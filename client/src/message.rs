use serde::{Serialize, Deserialize};

use super::testing;

mod message;


#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Connect { user_name: String },
    Quit {user_name: String},
}

