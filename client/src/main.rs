use std::io::prelude::*;
use std::net::TcpStream;

use std::env;

mod testing;

// extern crate common;
// use common::another;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();
    let mut stream = TcpStream::connect(config.server_address_and_port)?;

    let mymessage = common::ClientMessage::Connect {
        user_name: String::from("Lukas"),
    };
    let mut serialized = serde_json::to_vec(&mymessage).unwrap();
    let _result = stream.write(&mut serialized);

    loop {}

    // let mymessage = common::Message::Quit{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&mymessage).unwrap();
    // let _result = stream.write(&mut serialized);

    // let args: Vec<String> = env::args().collect();

    // if args.len() != 2 {
    //     panic!("usage: cmd [filename.yaml]");
    // }

    // let user_actions = testing::read_user_actions(&args[1]).unwrap();

    // for message_and_duration in user_actions.iter() {
    //     println!("{:?}", message_and_duration.message);
    //     println!("duration: {}s", message_and_duration.duration);
    // }

    Ok(())
}
