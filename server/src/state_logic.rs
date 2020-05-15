use std::sync::{Arc, Mutex};

use std::thread;
use std::time;


use rand::prelude::*;

extern crate common;
use common::{ServerMessage, ConnectionStatus, ServerState, PartyMembership};
use common::LegisationSubState;

pub fn all_players_ready(players: Vec<common::Player>) -> bool {
    let ready_count = players.iter().filter(|player| player.ready == true ).count();
    let relevant_count = players.iter().
        filter(|player| 
            (player.connection_status == ConnectionStatus::Connected) &&
            (player.status == common::PlayerState::Alive)
    
    ).count();
    (ready_count >= relevant_count)
}

fn get_next_president_index(president_id: String, players: &Vec<common::Player>) -> usize {
    let president_index = players.iter().position(|p| p.player_id == president_id).unwrap();
    let mut next_president_index = president_index;
    loop {
        next_president_index = (next_president_index + 1) % players.len();
        match players[next_president_index].status { // only select alive presidents ...
            common::PlayerState::Alive => { return next_president_index },
            _ => {}
        }
    }
}



pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            println!("{:?}", data.state);
            println!("{:?}", data.shared.players);

            // check win conditions
            if data.shared.fascist_policies_count >= 6 {
               data.state = ServerState::GameOver{winner: PartyMembership::Fascist};
            }
            if data.shared.liberal_policies_count >= 5 {
                data.state = ServerState::GameOver{winner: PartyMembership::Liberal};
            }

            // TODO add more win conditions here

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
            
            let liberal_policies_count = data.shared.liberal_policies_count;
            let fascist_policies_count = data.shared.fascist_policies_count;

            for player in data.shared.players.clone() {
                if player.connection_status == ConnectionStatus::Connected {
                    let state = data.state.clone();


                    match player.party_membership {
                        Some(common::PartyMembership::Fascist) => {
                            // if data.shared.fascist_known_by_hitler
                            if player.is_hitler.unwrap() == false || data.shared.fascist_known_by_hitler.unwrap() {
                                data.queue_message(
                                    player.thread_id,
                                    ServerMessage::StatusUpdate{players: current_players.clone(), state: state, player_id: Some(player.player_id),
                                        liberal_policies_count: liberal_policies_count,
                                        fascist_policies_count: fascist_policies_count,
                                    }
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
                                    ServerMessage::StatusUpdate{players: players_with_hidden_memberships, state: state, player_id: Some(player.player_id),
                                    liberal_policies_count: liberal_policies_count,
                                    fascist_policies_count: fascist_policies_count,
                                    }
                                );
                            }
                        },
                        Some(common::PartyMembership::Liberal) => {
                            let players = match state {
                                ServerState::GameOver{winner: _} => data.shared.players.clone(),
                                _ => {
                                    // hide party membership for liberals
                                    current_players.clone().iter_mut().map(|player_it| {
                                        if player.player_id != player_it.player_id {
                                            player_it.is_hitler = None;
                                            player_it.party_membership = None;
                                        }
                                        player_it.clone() // why do i need to clone here, rust?
                                    }).collect()
                                }
                            };
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::StatusUpdate{players: players, state: state, player_id: Some(player.player_id),
                                    liberal_policies_count: liberal_policies_count,
                                    fascist_policies_count: fascist_policies_count,
                                }
                            );
                        },
                        None => {
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::StatusUpdate{players: current_players.clone(), state: state, player_id: Some(player.player_id),
                                    liberal_policies_count: liberal_policies_count,
                                    fascist_policies_count: fascist_policies_count,}
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
                ServerState::Election {election_count, presidential_nominee, chancellor_nominee} => {
                    // TODO wait for ready of all players before moving to next state
                    let number_active_players = data.shared.players.iter().
                            filter(|player| 
                                (player.connection_status == ConnectionStatus::Connected) &&
                                (player.status == common::PlayerState::Alive)
                        
                        ).count();

                    if let Some(votes) = data.shared.votes.clone() {
                        let voted_count = votes.len();
                        let vote_complete = voted_count == number_active_players;

                        if vote_complete {
                            let ja_votes = votes.values()
                            .filter(|&vote| vote == &common::VoteState::Ja).count();
                            println!("{}/{}", ja_votes, votes.len());

                            if ja_votes > (number_active_players - ja_votes) {
                                // vote succeeded
                                println!("vote passed!");
                                data.shared.votes = None;
                                data.shared.election_count = 0;

                                // test if chancelor is hitler
                                if data.shared.fascist_policies_count >= 3 {
                                    let players = data.shared.players.clone();
                                    let chancellor = players.iter().find(|player| player.player_id == chancellor_nominee).unwrap();

                                    if chancellor.is_hitler.unwrap() {
                                        data.state = ServerState::GameOver{winner: PartyMembership::Fascist};
                                    } else {
                                        data.state = ServerState::LegislativeSession{president: presidential_nominee.clone(), 
                                        chancellor: chancellor_nominee.clone(), 
                                        substate: common::LegisationSubState::PresidentsChoice, waiting: false};
                                    }
                                } else {
                                    data.state = ServerState::LegislativeSession{president: presidential_nominee.clone(), 
                                        chancellor: chancellor_nominee.clone(), 
                                        substate: common::LegisationSubState::PresidentsChoice, waiting: false};
                                }


                            } else {
                                // vote failed
                                if election_count >= 3 {
                                    data.state = ServerState::Chaos{waiting: false, presidential_nominee: presidential_nominee.clone()};
                                    data.shared.votes = None;
                                    data.shared.election_count = 0;
                                    println!("go to chaos");

                                } else {
                                    let next_president_index = get_next_president_index(presidential_nominee.clone(), &data.shared.players);
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


                    



                },

                ServerState::LegislativeSession {president, chancellor, substate, waiting} => {
                    match substate {
                        LegisationSubState::PresidentsChoice => {
                            if waiting == false {
                                // check if policy pile is empty or < 3 -> shuffle discard pile into policy pile
                                if data.shared.draw_pile.len() < 3 {
                                    println!("reshuffled draw pile");
                                    let discard = data.shared.discard_pile.clone();
                                    data.shared.draw_pile.extend(discard);
                                    data.shared.discard_pile = Vec::new();
                                }

                                // pick policy cards, send policy cards to president
                                for _ in 0..3 {
                                    let card = data.shared.draw_pile.pop().unwrap(); // this should always work
                                    data.shared.current_cards.push(card);
                                }

                                let players = data.shared.players.clone();
                                let p = players.iter().find(|player| player.player_id == president).unwrap();
                                let cards_to_send = data.shared.current_cards.clone();
                                data.queue_message(p.thread_id, 
                                    ServerMessage::PolicyUpdate{cards: cards_to_send});
                                println!("sent policies to president");
                                data.state = ServerState::LegislativeSession {president, chancellor, substate, waiting: true};
                            } else {
                                // let pres_message = 
                                // TODO maybe this can be removed later
                                // resend policies in case client didn't get them
                                let players = data.shared.players.clone();
                                let p = players.iter().find(|player| player.player_id == president).unwrap();
                                let cards_to_send = data.shared.current_cards.clone();
                                data.queue_message(p.thread_id, 
                                    ServerMessage::PolicyUpdate{cards: cards_to_send});
                                println!("sent policies to president");

                                match data.shared.policies_received.len() {
                                    0 => {},
                                    2 => {
                                        // TODO verify that those cards are valid
                                        // let data.shared.current_cards
                                        for returned_card in data.shared.policies_received.clone() {
                                            let index = data.shared.current_cards.iter().position(|x| *x == returned_card).unwrap();
                                            data.shared.current_cards.remove(index);
                                        }
                                        let discard_card = data.shared.current_cards[0].clone();
                                        data.shared.discard_pile.push(discard_card); // put the remaining card to discard
                                        data.shared.current_cards = data.shared.policies_received.clone();
                                        
                                        data.shared.policies_received = Vec::new();
                                        

                                        data.state = ServerState::LegislativeSession {president, chancellor,
                                            substate: LegisationSubState::ChancellorsChoice, waiting: false};
                                        println!("going to chancelorschoice state");
                                    },
                                    _ => panic!("got invalid amount of policies {}", data.shared.policies_received.len()),
                                }
                            }
                            
                        },
                        LegisationSubState::ChancellorsChoice => {
                            // send policy cards to chancellor
                            if waiting == false {
                                let players = data.shared.players.clone();
                                let c = players.iter().find(|player| player.player_id == chancellor).unwrap();
                                let cards_to_send = data.shared.current_cards.clone();

                                data.queue_message(c.thread_id, 
                                    ServerMessage::PolicyUpdate{cards: cards_to_send});
                                data.state = ServerState::LegislativeSession {president, chancellor, substate, waiting: true};
                            } else {
                                //waiting == true
                                let players = data.shared.players.clone();
                                let c = players.iter().find(|player| player.player_id == chancellor).unwrap();
                                let cards_to_send = data.shared.current_cards.clone();
                            
                                data.queue_message(c.thread_id, 
                                    ServerMessage::PolicyUpdate{cards: cards_to_send});

                                println!("got {} policies", data.shared.policies_received.len());
                                match data.shared.policies_received.len() {
                                    0 => {},
                                    1 => {
                                        // TODO verify that those cards are valid
                                        // let data.shared.current_cards
                                        for returned_card in data.shared.policies_received.clone() {
                                            let index = data.shared.current_cards.iter().position(|x| *x == returned_card).unwrap();
                                            data.shared.current_cards.remove(index);
                                        }
                                        let discard_card = data.shared.current_cards[0].clone();
                                        data.shared.discard_pile.push(discard_card); // put the remaining card to discard
                                        data.shared.current_cards = data.shared.policies_received.clone();
                                        

                                        data.state = ServerState::LegislativeSession {president, chancellor,
                                            substate: LegisationSubState::Done, waiting: false};
                                        println!("chancellor selected a policy");
                                    },
                                    _ => {} // stability over correctness.. panic!("got invalid amount of policies {}", data.shared.policies_received.len()),
                                }
                            }
                            
                            // wait for policy respone
                            // put remaining card to discard pile
                            // enact policy -> there should be some kind of a win condition check here eventually
                        },
                        LegisationSubState::Done => {
                            // wait for players to get ready
                            // continue to nomination state
                            let current_cards = data.shared.current_cards.clone();

                            for player in data.shared.players.clone() {
                                data.queue_message(player.thread_id, 
                                    ServerMessage::PolicyUpdate{cards: current_cards.clone()});
                            }
                            if waiting {
                                // wait for players to get ready...
                                let online_count = data.shared.players.iter().
                                    filter(|player| player.connection_status == ConnectionStatus::Connected).count();
                                if all_players_ready(data.shared.players.clone()) && online_count >= 1 {// minimum players should be changed to 5
                                    
                                    let next_president_index = get_next_president_index(president.clone(), &data.shared.players);
                                    
                                    let enacted_policy = data.shared.current_cards[0].clone();
                                    data.shared.current_cards = Vec::new();
                                    data.shared.policies_received = Vec::new();
                                    data.shared.players = data.shared.players.iter_mut().
                                            map(|player| {player.ready = false; player.clone()}).collect();


                                    // decided where to go from here 
                                    // if fascist policy, check if legislative action is next
                                    match (enacted_policy, data.shared.player_number.unwrap(), data.shared.fascist_policies_count) {
                                        
                                        (common::PolicyCard::Fascist, 3, 3) => {
                                            data.state = ServerState::PolicyPeek{
                                                president: president,
                                                chancellor: chancellor,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 3, 4) => {
                                            data.state = ServerState::Execution{
                                                president,
                                                chancellor,
                                                executed: false,
                                                victim: None,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 5, 4) => {
                                            data.state = ServerState::Execution{
                                                president,
                                                chancellor,
                                                executed: false,
                                                victim: None,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 5, 5) => {
                                            data.state = ServerState::Execution{
                                                president,
                                                chancellor,
                                                executed: false,
                                                victim: None,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 6, 4) => {
                                            data.state = ServerState::Execution{
                                                president,
                                                chancellor,
                                                executed: false,
                                                victim: None,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 6, 5) => {
                                            data.state = ServerState::Execution{
                                                president,
                                                chancellor,
                                                executed: false,
                                                victim: None,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 5, 3) => {
                                            data.state = ServerState::PolicyPeek{
                                                president: president,
                                                chancellor: chancellor,
                                            };
                                        },
                                        (common::PolicyCard::Fascist, 6, 3) => {
                                            data.state = ServerState::PolicyPeek{
                                                president: president,
                                                chancellor: chancellor,
                                            };
                                        },
                                        // otherwise if nothing matches go to nomination
                                        (common::PolicyCard::Fascist, _, _) => {
                                            data.state = ServerState::Nomination{
                                                last_president: Some(president),
                                                last_chancellor: Some(chancellor),
                                                presidential_nominee: data.shared.players[next_president_index].player_id.clone()
                                            };
                                        },
                                        (common::PolicyCard::Liberal, _, _) => {
                                            data.state = ServerState::Nomination{
                                                last_president: Some(president),
                                                last_chancellor: Some(chancellor),
                                                presidential_nominee: data.shared.players[next_president_index].player_id.clone()
                                            };
                                        },
                                    }

                                    
                                    
                                }

                            } else {
                                match current_cards[0] {
                                    common::PolicyCard::Fascist => data.shared.fascist_policies_count += 1,
                                    common::PolicyCard::Liberal => data.shared.liberal_policies_count += 1,
                                }

                                data.state = ServerState::LegislativeSession {president, chancellor,
                                    substate: LegisationSubState::Done, waiting: true};
                            }
                        }
                    }
                }
                ServerState::PolicyPeek{president, chancellor} => {
                    let players = data.shared.players.clone();
                    let p = players.iter().find(|player| player.player_id == president).unwrap();

                    
                    if data.shared.draw_pile.len() < 3 {
                        println!("reshuffled draw pile");
                        let discard = data.shared.discard_pile.clone();
                        data.shared.draw_pile.extend(discard);
                        data.shared.discard_pile = Vec::new();
                    }
                    let cards_to_send: Vec<&common::PolicyCard> = data.shared.draw_pile.iter().rev().take(3).collect();
                    let cards_to_send = cards_to_send.iter().map(|&p| (*p).clone()).collect();
                    data.queue_message(p.thread_id, 
                        ServerMessage::PolicyUpdate{cards: cards_to_send});
                    println!("sent policies to president");

                    // if ready...
                    let p = players.iter().find(|player| player.player_id == president).unwrap();
                    if p.ready {

                        data.shared.players = data.shared.players.iter_mut().
                        map(|player| {player.ready = false; player.clone()}).collect();

                        let next_president_index = get_next_president_index(president.clone(), &data.shared.players);

                        data.state = ServerState::Nomination{
                            last_president: Some(president),
                            last_chancellor: Some(chancellor),
                            presidential_nominee: data.shared.players[next_president_index].player_id.clone()
                        };
                    }
                    
                }
                ServerState::Chaos{waiting, presidential_nominee} => {
                    if waiting == false {
                        if data.shared.draw_pile.len() < 1 {
                            println!("reshuffled draw pile");
                            let discard = data.shared.discard_pile.clone();
                            data.shared.draw_pile.extend(discard);
                            data.shared.discard_pile = Vec::new();
                        }
    
                        // pick policy cards, send policy cards to president

                        let card = data.shared.draw_pile.pop().unwrap(); // this should always work
                        
                        match card {
                            common::PolicyCard::Fascist => data.shared.fascist_policies_count += 1,
                            common::PolicyCard::Liberal => data.shared.liberal_policies_count += 1,
                        }
                        
                        data.state = ServerState::Chaos{waiting: true, presidential_nominee};
                    } else {
                        // wait for ready here then if all ready go to fresh nomination
                        if all_players_ready(data.shared.players.clone()) {
                            let next_president_index = get_next_president_index(presidential_nominee.clone(), &data.shared.players);

                            data.state = ServerState::Nomination{last_president: None, last_chancellor: None, presidential_nominee: data.shared.players[next_president_index].player_id.clone()};
                        
                            // TODO create function to set all players to not ready
                            data.shared.players = data.shared.players.iter_mut().
                            map(|player| {player.ready = false; player.clone()}).collect();
                        }
                    }
                    
                },
                ServerState::Execution{president, victim, executed, chancellor} => {
                    if executed {
                        // set executed state of victim here
                        let victim = victim.unwrap();
                        let victim_player = data.
                            shared.players.iter_mut().find(|player| player.player_id == victim).unwrap();
                        victim_player.status = common::PlayerState::Dead;

                        let hitler = data.shared.players.iter().find(|player| player.is_hitler.unwrap()).unwrap();


                        if victim == hitler.player_id {
                            // hitler executed, liberals win
                            data.state = ServerState::GameOver{winner: PartyMembership::Liberal};
                        } else {
                            // executed player was not hitler, just continue with next nomination after players are ready
                            let online_count = data.shared.players.iter().
                                    filter(|player| player.connection_status == ConnectionStatus::Connected).count();
                            if all_players_ready(data.shared.players.clone()) && online_count >= 1 {
                                let next_president_index = get_next_president_index(president.clone(), &data.shared.players);

                                data.shared.players = data.shared.players.iter_mut().
                                    map(|player| {player.ready = false; player.clone()}).collect();

                                data.state = ServerState::Nomination{
                                    last_chancellor: Some(chancellor),
                                    last_president: Some(president),
                                    presidential_nominee: data.shared.players[next_president_index].player_id.clone()
                                };
                            }
                        }
                    }
                },

                
                _ => {}
            }

        }


        thread::sleep(time::Duration::from_millis(500));
    }
}   