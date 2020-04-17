use serde::{Deserialize, Serialize};

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