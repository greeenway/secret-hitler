use std::net::{TcpListener};
use std::sync::{Arc, Mutex};

use std::thread;

mod state;
mod communicate;
mod state_logic;


fn main() -> std::io::Result<()> {
    
    let data = Arc::new(Mutex::new(
        state::GameState {
            state: state::State::Pregame,
            players: Vec::new(),
        }
    ));

    let state_data = Arc::clone(&data);

    thread::spawn( move || {
        let _result = state_logic::handle_state(state_data);
    });


    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for (id, stream) in listener.incoming().enumerate() {

        let stream = stream.unwrap();
        let data = Arc::clone(&data);

        thread::spawn(move || {
            let _ = communicate::handle_thread(id, stream, data);
        });
    }

    Ok(())
}
