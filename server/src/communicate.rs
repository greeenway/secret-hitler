use std::io::prelude::*;

use std::thread;
use std::time;

use std::net::{TcpStream};
use std::sync::{Arc, Mutex};

use serde::{Deserialize};
use serde_json;

extern crate common;

// todo user connection
// we need a mapping thread id -> player_id (username)
// each player has a connection_status connected/disconnected
// player connects: id gets mapped to player_id, server verifies id is unique
// if player is disconnected reconnect -> connection_status -> connected
// map new thread id to player_id
// what happens if a user is disconnected? can the game continue? how long to wait?
// structures
// players vector, has connection_status
// thread_to_player_id 
// todo: how to handle disconnects? with a message? (towards server state)


pub fn handle_thread(id: usize, mut stream: TcpStream, data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(200)));
    let user_timeout = time::Duration::from_secs(20);
    let mut last_alive = time::Instant::now();
    let mut done = false;
    while !done {
        { // lock mutex
            let mut data = data.lock().unwrap();
            
            // read message from client
            let read_stream = stream.try_clone()?;
            let mut de = serde_json::Deserializer::from_reader(read_stream);
            
            loop {
                // parse all messages from client
                let result = common::ClientMessage::deserialize(&mut de);
                if let Ok(message) = result {
                    if message == common::ClientMessage::StillAlive {
                        last_alive = time::Instant::now();
                    }
                    println!("client sent: {:?}", message);
                    crate::state::update_state(&mut *data, message, id);
                } else {
                    break;
                }
            } 

            let elapsed_since_alive = last_alive.elapsed();
            println!("thread {}: time since last alive: {}ms", id,
                elapsed_since_alive.as_millis());

            if elapsed_since_alive > user_timeout {
                println!("user thread {} timed out", id);
                done = true;
                let mut serialized = serde_json::to_vec(&common::ServerMessage::Kicked{reason: String::from("No alive received")})
                .unwrap();
                let _result = stream.write(&mut serialized);
            }

            // data.players.push(id as i32 + 1);
            // let message = &*data ;//.outboxes.get(&id).unwrap().pop_back();

            // let message = (*data).outboxes.get(&id).unwrap().pop_back(); //.outboxes.get(&id).unwrap().pop_back();

            data.queue_message(id, common::ServerMessage::Hello{message: String::from("hi enq client!")});
            // let mut serialized = serde_json::to_vec(&*data).unwrap();
            while let Some(message) = data.pop_message(id) {
                let mut serialized = serde_json::to_vec(&message).unwrap();
                let _result = stream.write(&mut serialized);
            }

            

            // let mut serialized = serde_json::to_vec(&common::ServerMessage::Hello{message: String::from("hi client!")})
            //     .unwrap();
            // let _result = stream.write(&mut serialized);

            

        }
        thread::sleep(time::Duration::from_millis(1000));
        
    }
    Ok(())
}

