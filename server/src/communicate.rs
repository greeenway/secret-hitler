use std::io::prelude::*;

use std::thread;
use std::time;

use std::net::{TcpStream};
use std::sync::{Arc, Mutex};

use serde::{Deserialize};
use serde_json;

extern crate common;


pub fn handle_thread(id: usize, mut stream: TcpStream, data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(200)));
    
    loop {
        { // lock mutex
            let mut data = data.lock().unwrap();
            
            // read message from client
            let read_stream = stream.try_clone()?;
            let mut de = serde_json::Deserializer::from_reader(read_stream);
            
            loop {
                // parse all messages from client
                let result = common::ClientMessage::deserialize(&mut de);
                if let Ok(message) = result {
                    println!("client sent: {:?}", message);
                    crate::state::update_state(&mut *data, message);
                } else {
                    break;
                }
            }   

            // send state to client
            data.players.push(id as i32 + 1);

            // let mut serialized = serde_json::to_vec(&*data).unwrap();
            let mut serialized = serde_json::to_vec(&common::ServerMessage::Hello{message: String::from("hi client!")})
                .unwrap();
            let _result = stream.write(&mut serialized);

        }
        thread::sleep(time::Duration::from_millis(1000));
        
    }
}

