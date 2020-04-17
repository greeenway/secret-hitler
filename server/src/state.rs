use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    Pregame,
    GameOver,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub state: State,
    pub players: Vec<i32>,
}

pub fn update_state(state: &mut crate::state::GameState, message: common::ClientMessage) {
    println!("update state using message: {:?}", message);
    state.players.push(91);
}