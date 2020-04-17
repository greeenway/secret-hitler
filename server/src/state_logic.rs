use std::sync::{Arc, Mutex};

use std::thread;
use std::time;

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let data = data.lock().unwrap();
            println!("{:?}", data.state);
            println!("{:?}", data.players);
        }
        thread::sleep(time::Duration::from_millis(2000));
    }
}