use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use common::ServerMessage;
use common::ConnectionStatus;
use common::Player;
use common::ServerState;
use common::VoteState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedState {
    pub players: Vec<Player>,
    pub outboxes: HashMap<usize, VecDeque<ServerMessage>>,
    pub fascist_known_by_hitler: Option<bool>,
    pub player_number: Option<u8>,
    pub votes: Option<HashMap<String, VoteState>>,
}

impl SharedState {
    pub fn new(_: common::Configuration) -> SharedState {
        SharedState {
            players: Vec::new(),
            outboxes: HashMap::new(),
            fascist_known_by_hitler: None,
            player_number: None,
            votes: None,
        }
    }
}




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub state: ServerState,
    pub shared: SharedState,
}



impl GameState {
    pub fn new(config: common::Configuration) -> GameState {
        GameState {
            state: ServerState::Pregame,
            shared: SharedState::new(config),
        }
    }

    pub fn queue_message(&mut self, to_thread_id: usize, message: ServerMessage) {
        self.shared.outboxes.get_mut(&to_thread_id).unwrap().push_back(message);
    }

    pub fn pop_message(&mut self, thread_id: usize) -> Option<ServerMessage>{

        let res = self.shared.outboxes.get_mut(&thread_id);
        match res {
            Some(deque) => {
                deque.pop_back() 
            },
            None => None,
        }
    }

}

pub fn update_state(state: &mut crate::state::GameState, message: common::ClientMessage, id: usize) {
    
    // server message processing
    match message {
        common::ClientMessage::Connect{name} => {
            if !state.shared.outboxes.contains_key(&id) {
                state.shared.outboxes.insert(id, VecDeque::new());
            } 
            // else {
            //     // forbid user to change player name
            //     if let Some(player) = state.shared.players.iter().find(|player| player.thread_id == id) {
            //         let reason = format!("Already connected as {}", player.player_id);
            //         state.queue_message(id, ServerMessage::Rejected{reason: reason});
            //         return;
            //     }
            // }

            if let Some(player) = state.shared.players.iter_mut().find(|player| player.player_id == name) {
                if player.connection_status == ConnectionStatus::Disconnected {
                    // reconnect
                    player.connection_status = ConnectionStatus::Connected;
                    player.thread_id = id;
                    state.queue_message(id, ServerMessage::Reconnected{user_name: name, state: state.state.clone()});
                } else {
                    // user already present and connected
                    state.queue_message(id, ServerMessage::Kicked{reason: String::from("user already logged in")});
                }
            } else {
                state.shared.players.push(Player::new(name.clone(), id));
                state.queue_message(id, 
                    ServerMessage::Connected { user_name: name });
            }
        },
        common::ClientMessage::Ready{ready} => {
            if let Some(player) = state.shared.players.iter_mut().find(|player| player.thread_id == id) {
                player.ready = ready;
            }
        },
        common::ClientMessage::Nominated{chancellor_nominee} => {
            match state.state.clone() {
                ServerState::Nomination{last_president, last_chancellor, presidential_nominee} => {
                    state.state = ServerState::Election{fail_count: 0, presidential_nominee, chancellor_nominee}
                },
                _ => {}
            }
            
        }
        common::ClientMessage::Hello => {
            println!("user thread {} says hello.", id);
        },
        common::ClientMessage::Chat { message } => {
            let user_name = state
                .shared
                .players
                .iter()
                .find(|player| player.thread_id == id)
                .unwrap()
                .player_id
                .clone();

            for player in state.shared.players.clone() {
                if player.connection_status == ConnectionStatus::Connected {
                    state.queue_message(
                        player.thread_id,
                        ServerMessage::Chat {
                            user_name: user_name.clone(),
                            message: message.clone(),
                        },
                    );
                }
            }
        }
        common::ClientMessage::Vote { selected, player_id} => {
            // TODO check valid state here (meaning election)
            if state.shared.votes == None {
                state.shared.votes = Some(HashMap::new());
            }
            if state.shared.votes != None { // this should always be true...
                let mut votes = state.shared.votes.clone().unwrap();
                votes.insert( player_id, selected); // TODO: find a better way, than creating the hashmap again...
                state.shared.votes = Some(votes);
            }
        }
        common::ClientMessage::StillAlive => {
            // ignore here, as it is handled by communicate module
        },
        message => println!("don't know how to handle '{:?}' yet.", message),
    }

}
