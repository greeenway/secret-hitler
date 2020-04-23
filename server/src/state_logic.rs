use std::sync::{Arc, Mutex};

use std::thread;
use std::time;


extern crate common;
use crate::state::State;
use common::{ServerMessage, ConnectionStatus};

pub fn handle_state(data: Arc<Mutex<crate::state::GameState>>) -> std::io::Result<()> {
    loop {
        {
            // let mut data = data.lock().unwrap();
            let mut data = data.lock().unwrap();

            println!("{:?}", data.state);
            println!("{:?}", data.shared.players);
            let current_players = data.shared.players.clone();
            for player in data.shared.players.clone() {
                if player.connection_status == ConnectionStatus::Connected {
                    data.queue_message(
                        player.thread_id,
                        ServerMessage::StatusUpdate{players: current_players.clone()}
                    );
                }
            }

            match data.state {
                State::Pregame => {
                    let ready_count = data.shared.players.iter().filter(|player| player.ready == true).count();
                    let online_count = data.shared.players.iter().
                        filter(|player| player.connection_status == ConnectionStatus::Connected).count();

                    if ready_count == online_count {
                        for player in data.shared.players.clone() {
                            data.queue_message(
                                player.thread_id,
                                ServerMessage::Advance,
                            );
                        }
                    }
                }
                _ => {}
            }

        }


        thread::sleep(time::Duration::from_millis(2000));
    }
}   