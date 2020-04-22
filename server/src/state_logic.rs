use std::sync::{Arc, Mutex};

use std::thread;
use std::time;

extern crate common;

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            println!("{:?}", data.state);
            println!("{:?}", data.shared.players);
            let current_players = data.shared.players.clone();
            for player in data.shared.players.clone() {
                if player.connection_status == common::ConnectionStatus::Connected {
                    data.queue_message(
                        player.thread_id, // TODO send to all online threads
                        common::ServerMessage::StatusUpdate{players: current_players.clone()}
                    );
                }
                
            }
        }


        thread::sleep(time::Duration::from_millis(2000));
    }
}   