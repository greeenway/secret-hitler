use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use common::ServerMessage;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub connection_status: ConnectionStatus,
    pub thread_id: usize,
}

impl Player {
    pub fn new(player_id: String, thread_id: usize) -> Player {
        Player {
            player_id: player_id,
            connection_status: ConnectionStatus::Connected,
            thread_id: thread_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    Pregame,
    GameOver,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub state: State,
    pub players: Vec<Player>,
    pub outboxes: HashMap<usize, VecDeque<ServerMessage>>,

}

impl GameState {
    pub fn new() -> GameState {
        let mut state = GameState {
            state: State::Pregame,
            players: Vec::new(),
            outboxes: HashMap::new(),
        };

        state.outboxes.insert(0, VecDeque::new());

        state
    }

    pub fn queue_message(&mut self, to_thread_id: usize, message: ServerMessage) {
        self.outboxes.get_mut(&to_thread_id).unwrap().push_back(message);
    }

    pub fn pop_message(&mut self, thread_id: usize) -> Option<ServerMessage>{

        let res = self.outboxes.get_mut(&thread_id);
        match res {
            Some(deque) => {
                deque.pop_back() 
            },
            None => None,
        }

        

    }

}

pub fn update_state(state: &mut crate::state::GameState, message: common::ClientMessage, id: usize) {
    // println!("update state using message: {:?}", message);

    match message {
        common::ClientMessage::Connect{name} => {
            if !state.outboxes.contains_key(&id) {
                state.outboxes.insert(id, VecDeque::new());
            } else {
                // forbid user to change player name
                if let Some(player) = state.players.iter().find(|player| player.thread_id == id) {
                    let reason = format!("Already connected as {}", player.player_id);
                    state.queue_message(id, ServerMessage::Rejected{reason: reason});
                    return;
                }
            }

            if let Some(player) = state.players.iter_mut().find(|player| player.player_id == name) {
                if player.connection_status == ConnectionStatus::Disconnected {
                    // reconnect
                    player.connection_status = ConnectionStatus::Connected;
                    player.thread_id = id;
                    state.queue_message(id, ServerMessage::Reconnected{user_name: name});
                } else {
                    // user already present and connected
                    state.queue_message(id, ServerMessage::Kicked{reason: String::from("user already logged in")});
                }
            } else {
                state.players.push(Player::new(name.clone(), id));
                state.queue_message(id, 
                    ServerMessage::Connected { user_name: name });
            }
        },
        common::ClientMessage::Hello => {
            println!("user thread {} says hello.", id);
        },
        common::ClientMessage::StillAlive => {
            // ignore here, as it is handled by communicate module
        },
        message => println!("don't know how to handle '{:?}' yet.", message),
    }

}