use std::sync::{Arc, Mutex};

use std::thread;
use std::time;


extern crate common;
use crate::state::State;
use common::{ServerMessage, ConnectionStatus};

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            println!("{:?}", data.state);
            println!("{:?}", data.shared.players);



            let current_players = data.shared.players.clone();


            for player in data.shared.players.clone() {
                if player.connection_status == ConnectionStatus::Connected {

                    match player.party_membership {
                        Some(common::PartyMembership::Fascist) => {
                            // if data.shared.fascist_known_by_hitler
                            if player.is_hitler.unwrap() == false || data.shared.fascist_known_by_hitler.unwrap() {
                                data.queue_message(
                                    player.thread_id,
                                    ServerMessage::StatusUpdate{players: current_players.clone()}
                                );
                                println!("players {:?}", current_players);
                                println!("sent to fascist or hitler {}", player.player_id);
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
                                    ServerMessage::StatusUpdate{players: players_with_hidden_memberships}
                                );
                                println!("sent to hitler {}", player.player_id);
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
                                ServerMessage::StatusUpdate{players: players_with_hidden_memberships}
                            );
                            println!("sent to liberal {}", player.player_id);
                        },
                        None => {
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::StatusUpdate{players: current_players.clone()}
                            );
                        },
                    }

                    

                }
            }

            match data.state {
                State::Pregame => {
                    let ready_count = data.shared.players.iter().filter(|player| player.ready == true).count();
                    let online_count = data.shared.players.iter().
                        filter(|player| player.connection_status == ConnectionStatus::Connected).count();

                    if ready_count == online_count && online_count >= 1 { // TODO later this should be changed to 5 
                        // fix player count
                        data.shared.player_number = Some(online_count as u8);

                        for player in data.shared.players.clone() {
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::Advance,
                            );
                        }
                        data.state = State::IdentityAssignment{identities_assigned: false};
                        data.shared.players = data.shared.players.iter_mut().
                            map(|player| {player.ready = false; player.clone()}).collect();
                    }
                },
                State::IdentityAssignment {identities_assigned } => {
                    // for now use this as a proxy to figure out if we already assigned roles
                    if identities_assigned == false {
                        let mut fascist_number = 0;
                        let mut liberal_number = 0;
    
    
                        let player_number =  data.shared.player_number.unwrap();
    
                        match player_number {
                            1 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                fascist_number = 1;
                            },
                            2 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                fascist_number = 1;
                            },
                            3 => {
                                // invalid only for debugging
                                data.shared.fascist_known_by_hitler = Some(true);
                                fascist_number = 2;
                            },
                            5 => {
                                data.shared.fascist_known_by_hitler = Some(true);
                                fascist_number = 2;
                            },
                            6 => {
                                data.shared.fascist_known_by_hitler = Some(true);
                                fascist_number = 2;
                            },
                            7 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                fascist_number = 3;
                            },
                            8 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                fascist_number = 3;
                            },
                            9 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                fascist_number = 4;
                            }
                            10 => {
                                data.shared.fascist_known_by_hitler = Some(false);
                                fascist_number = 4;
                            },
                            _ => panic!("This should never happen: player count {}", player_number),
                        }
                        liberal_number = player_number - fascist_number;
    
                        // assign memberships
                        match player_number {
                            1 => {
                                // debug only
                                data.shared.players[0].is_hitler = Some(true);
                                data.shared.players[0].party_membership = Some(common::PartyMembership::Fascist);   
                            },
                            2 => {
                                // debug only
                                data.shared.players[0].is_hitler = Some(true);
                                data.shared.players[0].party_membership = Some(common::PartyMembership::Fascist);
                                data.shared.players[1].is_hitler = Some(false);
                                data.shared.players[1].party_membership = Some(common::PartyMembership::Fascist);
                            },
                            3 => {
                                // debug only
                                data.shared.players[0].is_hitler = Some(true);
                                data.shared.players[0].party_membership = Some(common::PartyMembership::Fascist);
                                data.shared.players[1].is_hitler = Some(false);
                                data.shared.players[1].party_membership = Some(common::PartyMembership::Fascist);
                                data.shared.players[2].is_hitler = Some(false);
                                data.shared.players[2].party_membership = Some(common::PartyMembership::Liberal);
                            },
    
                            _ => {},
                        }
                        data.state = State::IdentityAssignment {identities_assigned: true };
                    }


                }
                _ => {}
            }

        }


        thread::sleep(time::Duration::from_millis(2000));
    }
}   