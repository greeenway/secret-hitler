use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Connect { user_name: String },
    Quit {user_name: String},
}

