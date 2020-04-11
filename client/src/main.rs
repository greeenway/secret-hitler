use std::io::prelude::*;
use std::net::TcpStream;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Connect { user_name: String },
    Quit {user_name: String},
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    let message = Message::Connect{user_name: String::from("Lukas")};
    let mut serialized = serde_json::to_vec(&message).unwrap();
    let _result = stream.write(&mut serialized);
    
    let message = Message::Quit{user_name: String::from("Lukas")};
    let mut serialized = serde_json::to_vec(&message).unwrap();
    let _result = stream.write(&mut serialized);

    Ok(())
    //test
} // the stream is closed here

