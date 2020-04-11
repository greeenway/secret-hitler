use std::io::prelude::*;

use serde::{Serialize, Deserialize};
use std::net::TcpListener;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Connect { user_name: String },
    Quit {user_name: String},
}

fn main() -> std::io::Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let listener = TcpListener::bind("127.0.0.1:34254").unwrap();



    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                
                // let mut buffer: Vec<u8> = Vec::new();
            
                // // stream.write(&[1])?;
                // stream.read(&mut buffer)?;
            
                let mut de = serde_json::Deserializer::from_reader(stream);
                
                
                loop {

                    // let read_bytes = stream.peek(&mut [0, 1000]).unwrap();


                    

                    let result = Message::deserialize(&mut de);

                    if let Ok(message) = result {
                        println!("{:?}", message);

                            match message {
                                Message::Connect{user_name} => println!("user {} received!", user_name),
                                _ => println!("something else was received!"),
                            }
                    } else {
                        // println!("didn't get anything!");
                    }

                    // match result {
                    //     Result::Message(message) => {
                    //         println!("{:?}", message);

                    //         match message {
                    //             Message::Connect{user_name} => println!("user {} received!", user_name),
                    //             _ => println!("something else was received!"),
                    //         }
                    //     },
                    //     // serde_json::error::Error
                    //     _ => {
                    //         println!("didn't get anything!");
                    //     }
                    // }


                }
                

            }
            Err(_e) => { /* connection failed */ }
        }
    }

    Ok(())

}