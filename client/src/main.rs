use std::io::prelude::*;
use std::net::TcpStream;

use std::env;

mod testing;
mod message;



fn main() -> std::io::Result<()> {
    // let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    // let message = Message::Connect{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&message).unwrap();
    // let _result = stream.write(&mut serialized);

    // let message = Message::Quit{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&message).unwrap();
    // let _result = stream.write(&mut serialized);

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: cmd [filename.yaml]");
    }

    let user_actions = testing::read_user_actions(&args[1]).unwrap();

    for message_and_duration in user_actions.iter() {
        println!("{:?}", message_and_duration.message);
        println!("duration: {}s", message_and_duration.duration);
    }


    Ok(())
}

