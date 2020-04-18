use std::sync::{Arc, Mutex};

use std::thread;
use std::time;

extern crate common;

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            data.queue_message(0, common::ServerMessage::Hello{message: String::from("hello from queue")});
            
            println!("{:?}", data.state);
            println!("{:?}", data.players);
        }
        thread::sleep(time::Duration::from_millis(2000));
    }
}   