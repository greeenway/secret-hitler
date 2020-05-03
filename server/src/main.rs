use std::net::{TcpListener};
use std::sync::{Arc, Mutex};

use std::thread;
use std::env;

mod state;
mod communicate;
mod state_logic;
mod seeds;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();

    // clean start
    // let game_state = state::GameState::new(config.clone());

    // seed game state
    let game_state = seeds::seed_game_state(config.clone(), "legislative_session");
    // let game_state = seeds::seed_game_state(config.clone(), "nomination");
    
    // TODO save server state regularly
    // TODO reload from server state
    
    

    let data = Arc::new(Mutex::new(game_state));
    

    let state_data = Arc::clone(&data);

    thread::spawn( move || {
        let _result = state_logic::handle_state(state_data);
    });

    println!("Listening on {}...", config.server_listen_address_and_port);
    let listener = TcpListener::bind(config.server_listen_address_and_port).unwrap();
    for (id, stream) in listener.incoming().enumerate() {

        let stream = stream.unwrap();
        let data = Arc::clone(&data);

        thread::spawn(move || {
            let _ = communicate::handle_thread(id, stream, data);
        });
    }

    Ok(())
}
