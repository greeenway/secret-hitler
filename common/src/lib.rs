use serde::{Serialize, Deserialize};

// pub mod another;
pub mod another;

pub fn say() -> another::Hello {
    another::Hello{x:2}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Connect { user_name: String },
    Quit {user_name: String},
}
