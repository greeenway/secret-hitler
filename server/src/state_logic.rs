use std::sync::{Arc, Mutex};

use std::thread;
use std::time;


use rand::prelude::*;

extern crate common;
use common::{ServerMessage, ConnectionStatus, ServerState};

pub fn all_players_ready(players: Vec<common::Player>) -> bool {
    let ready_count = players.iter().filter(|player| player.ready == true).count();
    let online_count = players.iter().
        filter(|player| player.connection_status == ConnectionStatus::Connected).count();
    (ready_count == online_count)
}

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            println!("{:?}", data.state);
            println!("{:?}", data.shared.players);



            let mut current_players = data.shared.players.clone();

            // map votes to players values if present
            if data.shared.votes != None {
                current_players = current_players.iter_mut().
                map(|player| {
                    // if player.player_id
                    match data.shared.votes.clone().unwrap().get(&player.player_id) {
                        Some(vote) => {player.vote = Some(vote.clone()); player.clone()},
                        None => {player.vote = None; player.clone()},
                    }
                }).collect();
            }
            


            for player in data.shared.players.clone() {
                if player.connection_status == ConnectionStatus::Connected {
                    let state = data.state.clone();


                    match player.party_membership {
                        Some(common::PartyMembership::Fascist) => {
                            // if data.shared.fascist_known_by_hitler
                            if player.is_hitler.unwrap() == false || data.shared.fascist_known_by_hitler.unwrap() {
                                data.queue_message(
                                    player.thread_id,
                                    ServerMessage::StatusUpdate{players: current_players.clone(), state: state, player_id: Some(player.player_id)}
                                );
                            } else {
                                // hide party member ships for hitler
                                let players_with_hidden_memberships = current_players.clone().iter_mut().map(|player_it| {
                                    if player.player_id != player_it.player_id {
                                        player_it.is_hitler = None;
                                        player_it.party_membership = None;
                                    }
                                    player_it.clone() // why do i need to clone here, rust?
                                }).collect();
                                data.queue_message(
                                    player.thread_id,
                                    ServerMessage::StatusUpdate{players: players_with_hidden_memberships, state: state, player_id: Some(player.player_id)}
                                );
                            }
                        },
                        Some(common::PartyMembership::Liberal) => {
                            // hide party membership for liberals
                            let players_with_hidden_memberships = current_players.clone().iter_mut().map(|player_it| {
                                if player.player_id != player_it.player_id {
                                    player_it.is_hitler = None;
                                    player_it.party_membership = None;
                                }
                                player_it.clone() // why do i need to clone here, rust?
                            }).collect();
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::StatusUpdate{players: players_with_hidden_memberships, state: state, player_id: Some(player.player_id)}
                            );
                        },
                        None => {
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::StatusUpdate{players: current_players.clone(), state: state, player_id: Some(player.player_id)}
                            );
                        },
                    }

                    

                }
            }

            match data.state.clone() {
                ServerState::Pregame => {
                    let online_count = data.shared.players.iter().
                        filter(|player| player.connection_status == ConnectionStatus::Connected).count();
                    if all_players_ready(data.shared.players.clone()) && online_count >= 1 {// minimum players should be changed to 5
                        // fix player count
                        data.shared.player_number = Some(online_count as u8);
                        
                        // TODO create helper function for this
                        // for player in data.shared.players.clone() {
                        //     data.queue_message(
                        //         player.thread_id,
                        //         ServerMessage::,
                        //     );
                        // }
                        data.state = ServerState::IdentityAssignment{identities_assigned: false};
                        data.shared.players = data.shared.players.iter_mut().
                            map(|player| {player.ready = false; player.clone()}).collect();
                    }
                },
                ServerState::IdentityAssignment {identities_assigned } => {
                    if identities_assigned == false {
                        let mut number_fascists = 0;
    
                        
                        let player_number =  data.shared.player_number.unwrap();
    
                        match player_number {
                            1 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                number_fascists = 1;
                            },
                            2 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                number_fascists = 1;
                            },
                            3 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                number_fascists = 2;
                            },
                            5 => {
                                data.shared.fascist_known_by_hitler = Some(true);
                                number_fascists = 2;
                            },
                            6 => {
                                data.shared.fascist_known_by_hitler = Some(true);
                                number_fascists = 2;
                            },
                            7 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                number_fascists = 3;
                            },
                            8 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                number_fascists = 3;
                            },
                            9 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                number_fascists = 4;
                            }
                            10 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                number_fascists = 4;
                            },
                            _ => panic!("This should never happen: player count {}, fascists {}", player_number, number_fascists),
                        }

    
                        // assign hitler and party memberships
                        let mut rng = rand::thread_rng();
                        let mut nums: Vec<u8> = (0..player_number).collect();
                        nums.shuffle(&mut rng);

                        for (i, num) in nums.iter().enumerate() {
                            if *num == 0 {
                                data.shared.players[i].is_hitler = Some(true);
                                data.shared.players[i].party_membership = Some(common::PartyMembership::Fascist);
                            } else if *num <= number_fascists - 1  {
                                data.shared.players[i].is_hitler = Some(false);
                                data.shared.players[i].party_membership = Some(common::PartyMembership::Fascist);
                            } else {
                                data.shared.players[i].is_hitler = Some(false);
                                data.shared.players[i].party_membership = Some(common::PartyMembership::Liberal);
                            }
                        }
                        
                        data.state = ServerState::IdentityAssignment {identities_assigned: true };


                    }

                    if all_players_ready(data.shared.players.clone()) {// minimum players should be changed to 5

                        
                        

                        let mut rng = rand::thread_rng();
                        let mut nums: Vec<u8> = (0..data.shared.player_number.unwrap()).collect();
                        nums.shuffle(&mut rng);
                        let first_nominee = data.shared.players[nums[0] as usize].player_id.clone(); // TODO pick random player more efficiently

                        data.state = ServerState::Nomination{last_president: None, last_chancellor: None, presidential_nominee: first_nominee.clone()};
                        
                        // TODO create function to set all players to not ready
                        data.shared.players = data.shared.players.iter_mut().
                        map(|player| {player.ready = false; player.clone()}).collect();

                        // for player in data.shared.players.clone() {
                        //     data.queue_message(
                        //         player.thread_id,
                        //         ServerMessage::AdvanceNomination{presidential_nominee: first_nominee.clone()},
                        //     );
                        // }
                        

                        // println!("sent advance to nomination!");
                    }

                },
                ServerState::Election {fail_count: _, presidential_nominee, chancellor_nominee} => {
                    // TODO wait for ready of all players before moving to next state
                    let number_of_players = data.shared.player_number.unwrap() as usize;

                    if let Some(votes) = data.shared.votes.clone() {
                        let voted_count = votes.len();
                        let vote_complete = voted_count == number_of_players;

                        if vote_complete {
                            let ja_votes = votes.values()
                            .filter(|&vote| vote == &common::VoteState::Ja).count();
                            println!("{}/{}", ja_votes, votes.len());

                            if ja_votes > (number_of_players - ja_votes) {
                                // vote succeeded
                                println!("vote passed!");
                                data.shared.votes = None;
                            } else {
                                // vote failed
                                // TODO check for chaos -> transition to chaos if fail count is high enough

                                let president_index = data.shared.players.iter().position(|p| p.player_id == presidential_nominee).unwrap();
                                let next_president_index = (president_index + 1) % number_of_players; // TODO use number of alive players

                                data.state = ServerState::Nomination{
                                    last_president: Some(presidential_nominee), 
                                    last_chancellor: Some(chancellor_nominee), 
                                    presidential_nominee: data.shared.players[next_president_index].player_id.clone()};
                                println!("vote failed!");
                                
                                data.shared.votes = None;

                            }
                        }
                    }


                    



                }
                _ => {}
            }

        }


        thread::sleep(time::Duration::from_millis(2000));
    }
}   