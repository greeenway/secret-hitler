// use std::io::prelude::*;
use std::io::{stdout, Write};

use std::net::TcpStream;

use std::thread;

use std::time;
use std::env;
use std::sync::{Arc, Mutex};

use crossterm::{queue, Result, 
    terminal::{SetSize, size, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::MoveTo,
};
use crossterm::style::{Print};
use crossterm::event::{Event, KeyEvent, KeyCode};//, KeyModifiers};

use serde::{Deserialize};

enum Show {
    PrintMessage,
    DontPrintMessage,
}

mod state;
use state::State;
// use common::ServerMessage;

mod render; // common render stuff
mod login_screen;
mod pre_game;
mod identity_assignment;
mod nomination;
mod election;
mod legislative_session;


// TODO: refactor main.rs of client into multiple files (per state? at least)


mod testing;

// extern crate common;
// use common::another;

fn send_message(mut stream: &TcpStream, message: common::ClientMessage, debug: Show) {
    let mut serialized = serde_json::to_vec(&message).unwrap();
    let _result = stream.write(&mut serialized);

    match debug {
        Show::PrintMessage => println!("sent: {:?}", message),
        _ => {},
    }
}

fn execute_command(command: String, mut data: std::sync::MutexGuard<'_, State>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts[..] {
        ["connect", user_name] => {
            data.shared.outbox.push_back(common::ClientMessage::Connect{name: String::from(user_name)});
        },
        ["hello"] => {
            data.shared.outbox.push_back(common::ClientMessage::Hello);
        },
        ["say", message] => {
            data.shared.outbox.push_back(common::ClientMessage::Chat {
                message: String::from(message),
            });
        },
        ["clear"] => data.shared.output.clear(),
        _ => {
            let lines = data.shared.max_cmd_lines;
            data.shared.output.push_front(format!("unknown command '{}'", command));
            data.shared.output.truncate(lines);
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();
    let stream = TcpStream::connect(config.clone().server_address_and_port)?;

    // let mut done = false;
    let write_stream = stream.try_clone()?;
    // let de = serde_json::Deserializer::from_reader(stream);

    let message = common::ClientMessage::Hello;
    send_message(&write_stream, message, Show::DontPrintMessage);
    let alive_stream = stream.try_clone()?;

    let client_state = Arc::new(Mutex::new(State::new(config.clone())));

    // let stop_alive_thread = Arc::new(Mutex::new(false));
    let stop_alive_copy = Arc::clone(&client_state);

    thread::spawn( move || {
        loop {
            {
                let client_state = stop_alive_copy.lock().unwrap();
                if client_state.shared.done {
                    break;
                }
            }
            send_message(&alive_stream, common::ClientMessage::StillAlive, Show::DontPrintMessage);
            thread::sleep(time::Duration::from_millis(4000));
        }
    });

    // start new code

    let (cols, rows) = size()?;

    enable_raw_mode().unwrap();
    // Resize terminal and scroll up.
    queue!(
        stdout(),
        SetSize(72, 22),
    )?;
    
    


    // rendering loop
    let render_mutex = Arc::clone(&client_state);
    let render_handle = thread::spawn(move|| {
        loop {
            {
                let mut data = render_mutex.lock().unwrap();
                if data.shared.done {
                    break;
                }
                let _res = queue!(stdout(), Clear(ClearType::All));

                data.draw();

                if data.shared.enable_debug_console {
                    for (line_number, line) in data.shared.output.iter().enumerate() {
                        let mut line_trucated = line.clone();
                        line_trucated.truncate(70);
                        let _res = queue!(stdout(), MoveTo(0, line_number as u16 + 1), Print(line_trucated));
                    }
    
                    if data.shared.cmd_prompt {
                        let _res = queue!(
                            stdout(),
                            MoveTo(0,0),
                            Print("> "),
                            Print(data.shared.input.clone()),
                        );
                    }
                }
            }
            let _res = stdout().flush();
            thread::sleep(std::time::Duration::from_millis(20));
        }
    });




    let event_mutex = client_state.clone();
    let event_handle = thread::spawn(move || {
        loop {
            match crossterm::event::read().unwrap() {
                Event::Key(event) => {
                    let mut data = event_mutex.lock().unwrap();

                    match event {
                        KeyEvent{
                            code: KeyCode::Char('0'),
                            modifiers: _,
                        } => {
                            if data.shared.enable_debug_console {
                                data.shared.cmd_prompt = !data.shared.cmd_prompt;
                            }
                        },
                        KeyEvent{
                            code: KeyCode::Esc,
                            modifiers: _,
                        } => {
                            data.shared.done = true;
                            break;
                        },
                        event => {
                            // handling of cmd_prompt here
                            if data.shared.cmd_prompt {
                                match event {
                                    KeyEvent{
                                        code: KeyCode::Char(c),
                                        modifiers: _,
                                    } => {
                                        data.shared.input = format!("{}{}", data.shared.input, c);
                                    }
                                    KeyEvent{
                                        code: KeyCode::Backspace,
                                        modifiers: _,
                                    } => {
                                        data.shared.input.pop();
                                    },
                                    KeyEvent{
                                        code: KeyCode::Enter,
                                        modifiers: _,
                                    } => {
                                        let input = data.shared.input.clone();
                                        data.shared.input.clear();
                                        execute_command(input, data);
                                    },
                                    _ => {},
                                }
                            } else {
                                data.handle_events(event);
                            } 
                        },
                    }
                    thread::sleep(std::time::Duration::from_millis(20));
                    
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
            
        }
    });

    // start thread mutex 
    let thread_mutex = client_state.clone();
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(50)));
    // let mut buffer = String::new();
    let mut de = serde_json::Deserializer::from_reader(stream);

    let thread_handle = thread::spawn(move || { 
        loop {
            {
                {
                    let mut data = thread_mutex.lock().unwrap();
                    if data.shared.done {
                        break;
                    }

                    loop {
                        // parse all messages from server
                        let result = common::ServerMessage::deserialize(&mut de);
                        
                        
                        if let Ok(message) = result {
                            data.shared.output.push_front(String::from(format!("{:?}", message)));
                            let lines = data.shared.max_cmd_lines;
                            data.shared.output.truncate(lines);
                            
                            match message {
                                common::ServerMessage::Kicked{reason} => {
                                    println!("got kicked from the server because '{}'", reason);
                                    data.shared.done = true;
                                },
                                _ => data.advance_handler(message)
                            }
                        } else {
                            break;
                        }
                    }

                    while let Some(message) = data.shared.outbox.pop_back() {
                        send_message(&write_stream, message, Show::PrintMessage)
                    }

                    
                }
                thread::sleep(time::Duration::from_millis(50));
            }
        }
        

    });

    // end thread mutex 

    render_handle.join().unwrap();
    event_handle.join().unwrap();
    thread_handle.join().unwrap();

    queue!(
        stdout(),
        Print("hallo"),
        Clear(ClearType::All),
    )?;



    // Be a good citizen, cleanup
    queue!(stdout(), MoveTo(0,0), SetSize(cols, rows))?;
    disable_raw_mode().unwrap();


    Ok(())
}
