use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    player_id: String,
    connection_status: ConnectionStatus,
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
    pub outboxes: HashMap<usize, VecDeque<common::ServerMessage>>,

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

    pub fn queue_message(&mut self, to_thread_id: usize, message: common::ServerMessage) {
        self.outboxes.get_mut(&to_thread_id).unwrap().push_back(message);
    }

    pub fn pop_message(&mut self, thread_id: usize) -> Option<common::ServerMessage>{

        let res = self.outboxes.get_mut(&thread_id);
        match res {
            Some(deque) => {
                deque.pop_back() 
            },
            None => None,
        }

        

    }

}

pub fn update_state(state: &mut crate::state::GameState, message: common::ClientMessage, _id: usize) {
    println!("update state using message: {:?}", message);

    match message {
        common::ClientMessage::Connect{name} => {
            state.players.push(
                Player {
                    player_id: name,
                    connection_status: ConnectionStatus::Connected,
                }
            );
        },
        message => println!("don't know how to handle '{:?}' yet.", message),
    }

}