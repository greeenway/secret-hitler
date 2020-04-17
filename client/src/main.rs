use std::io::prelude::*;
use std::io::{stdout, stdin};

use std::net::TcpStream;

use std::time;
use std::env;

use serde::{Deserialize};

mod testing;

// extern crate common;
// use common::another;

fn send_message(mut stream: &TcpStream, message: common::ClientMessage ) {
    let mut serialized = serde_json::to_vec(&message).unwrap();
    let _result = stream.write(&mut serialized);

    println!("sent: {:?}", message);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();
    let stream = TcpStream::connect(config.server_address_and_port)?;

    let write_stream = stream.try_clone()?;
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(200)));
    // let de = serde_json::Deserializer::from_reader(stream);

    let message = common::ClientMessage::Hello;
    send_message(&write_stream, message);
    
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(200)));
    let mut buffer = String::new();
    let mut de = serde_json::Deserializer::from_reader(stream);
    loop {
        loop {
            // parse all messages from server
            let result = common::ServerMessage::deserialize(&mut de);
            if let Ok(message) = result {
                println!("server sent: {:?}", message);
            } else {
                break;
            }
        }  

        print!("> ");
        let _= stdout().flush();
        buffer.clear();
        stdin().read_line(&mut buffer).expect("Did not enter a correct string");
        println!("got: {}",buffer);

        let pattern: Vec<&str> = buffer.split_whitespace().collect();

        match pattern.as_slice() {
            ["hello"] => send_message(&write_stream, common::ClientMessage::Hello),
            ["quit"] => send_message(&write_stream, common::ClientMessage::Quit),
            ["connect", user] => send_message(&write_stream, common::ClientMessage::Connect{name: String::from(*user)}),
            ["q"] => break,
            x => println!("unknown: {:?}", x),
        }
    }

    Ok(())
    



    // loop {
        
    //     let result = GameState::deserialize(&mut de);
    //     if let Ok(state) = result {
    //         println!("\nstate received:");
    //         println!("{:?}", state);

    //         let message = ClientMessage::Hello;
    //         let mut serialized = serde_json::to_vec(&message).unwrap();
    //         let _result = write_stream.write(&mut serialized);
    //         let _result = write_stream.write(&mut serialized);
    //         // let _result = write_stream.write(&mut serialized);
    //     } else {
    //         print!(".");
            
            
    //     }
    //     thread::sleep(time::Duration::from_millis(500));
    // }


    // let mymessage = common::ClientMessage::Connect {
    //     user_name: String::from("Lukas"),
    // };
    // let mut serialized = serde_json::to_vec(&mymessage).unwrap();
    // let _result = stream.write(&mut serialized);

    // loop {}



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
}
