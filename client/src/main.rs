use std::io::prelude::*;
use std::io::{stdout};

use std::net::TcpStream;

use std::thread;
use std::collections::VecDeque;
use std::time;
use std::env;
use std::sync::{Arc, Mutex};

use crossterm::{execute, Result, 
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

#[derive(PartialEq, Clone)]
struct GuiState {
    cmd_prompt: bool,
    input: String,
    output: VecDeque<String>,
    done: bool,
    max_cmd_lines: usize,
    inbox: VecDeque<common::ServerMessage>,
    outbox: VecDeque<common::ClientMessage>,
}

impl GuiState {
    pub fn new() -> GuiState {
        GuiState {
            cmd_prompt: true,
            input: String::from(""),
            output: VecDeque::new(),
            done: false,
            max_cmd_lines: 5,
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
        }
    }
}

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

fn execute_command(command: String, mut data: std::sync::MutexGuard<'_, GuiState>) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts[..] {
        ["connect", user_name] => {
            data.outbox.push_back(common::ClientMessage::Connect{name: String::from(user_name)});
        },
        ["hello"] => {
            data.outbox.push_back(common::ClientMessage::Hello);
        },
        ["clear"] => data.output.clear(),
        _ => {
            let lines = data.max_cmd_lines;
            data.output.push_front(format!("unknown command '{}'", command));
            data.output.truncate(lines);
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();
    let stream = TcpStream::connect(config.server_address_and_port)?;

    // let mut done = false;
    let write_stream = stream.try_clone()?;
    // let de = serde_json::Deserializer::from_reader(stream);

    let message = common::ClientMessage::Hello;
    send_message(&write_stream, message, Show::DontPrintMessage);
    let alive_stream = stream.try_clone()?;

    let gui_state = Arc::new(Mutex::new(GuiState::new()));

    // let stop_alive_thread = Arc::new(Mutex::new(false));
    let stop_alive_copy = Arc::clone(&gui_state);

    thread::spawn( move || {
        loop {
            {
                let gui_state = stop_alive_copy.lock().unwrap();
                if gui_state.done {
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
    execute!(
        stdout(),
        SetSize(60, 30),
    )?;
    
    

    // rendering loop
    let render_mutex = Arc::clone(&gui_state);
    let render_handle = thread::spawn(move|| {
        loop {
            {
                let data = render_mutex.lock().unwrap();
                if data.done {
                    break;
                }
                let _res = execute!(stdout(), Clear(ClearType::All));
                // let _res = execute!(stdout(), MoveTo(0,0), Print("Hallo Welt"));
                for (line_number, line) in data.output.iter().enumerate() {
                    let _res = execute!(stdout(), MoveTo(0, line_number as u16 + 1), Print(line));
                }

                if data.cmd_prompt {
                    let _res = execute!(
                        stdout(),
                        MoveTo(0,0),
                        Print("> "),
                        Print(data.input.clone()),
                    );
                }
            }
            thread::sleep(std::time::Duration::from_millis(20));
        }
    });


    let event_mutex = gui_state.clone();
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
                            data.cmd_prompt = !data.cmd_prompt;
                        },
                        KeyEvent{
                            code: KeyCode::Char(c),
                            modifiers: _,
                        } => {
                            if data.cmd_prompt {
                                data.input = format!("{}{}", data.input, c);
                            }
                        }
                        KeyEvent{
                            code: KeyCode::Backspace,
                            modifiers: _,
                        } => {
                            if data.cmd_prompt {
                                data.input.pop();
                            }
                        },
                        KeyEvent{
                            code: KeyCode::Enter,
                            modifiers: _,
                        } => {
                            if data.cmd_prompt {
                                let input = data.input.clone();
                                data.input.clear();
                                execute_command(input, data);
                            }
                        },
                        KeyEvent{
                            code: KeyCode::Esc,
                            modifiers: _,
                        } => {
                            data.done = true;
                            break;
                        },
                        _ => println!("another key"),
                    }
                    thread::sleep(std::time::Duration::from_millis(20));
                    
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
            
        }
    });

    // start thread mutex 
    let thread_mutex = gui_state.clone();
    let _ = stream.set_read_timeout(Some(time::Duration::from_millis(50)));
    // let mut buffer = String::new();
    let mut de = serde_json::Deserializer::from_reader(stream);

    let thread_handle = thread::spawn(move || { 
        loop {
            {
                {
                    let mut data = thread_mutex.lock().unwrap();
                    if data.done {
                        break;
                    }

                    loop {
                        // parse all messages from server
                        let result = common::ServerMessage::deserialize(&mut de);
                        if let Ok(message) = result {
                            data.output.push_back(String::from(format!("{:?}", message)));
                            if let common::ServerMessage::Kicked{reason} = message {
                                println!("got kicked from the server because '{}'", reason);
                                data.done = true;
                            }
                        } else {
                            break;
                        }
                    }

                    while let Some(message) = data.outbox.pop_back() {
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

    execute!(
        stdout(),
        Print("hallo"),
        Clear(ClearType::All),
    )?;



    // Be a good citizen, cleanup
    execute!(stdout(), MoveTo(0,0), SetSize(cols, rows))?;
    disable_raw_mode().unwrap();


    // new code end

    // let _ = stream.set_read_timeout(Some(time::Duration::from_millis(100)));
    // let mut buffer = String::new();
    // let mut de = serde_json::Deserializer::from_reader(stream);
    // while !done {
    //     loop {
    //         // parse all messages from server
    //         let result = common::ServerMessage::deserialize(&mut de);
    //         if let Ok(message) = result {
    //             println!("server sent: {:?}", message);
    //             if let common::ServerMessage::Kicked{reason} = message {
    //                 println!("got kicked from the server because '{}'", reason);
    //                 done = true;
    //             }
    //         } else {
    //             break;
    //         }
    //     }  
    //     if !done {
    //         print!("> ");
    //         let _= stdout().flush();
    //         buffer.clear();
    //         stdin().read_line(&mut buffer).expect("Did not enter a correct string");
    //         println!("got: {}",buffer);

    //         let pattern: Vec<&str> = buffer.split_whitespace().collect();

    //         match pattern.as_slice() {
    //             ["hello"] => send_message(&write_stream, common::ClientMessage::Hello, Show::PrintMessage),
    //             ["quit"] => send_message(&write_stream, common::ClientMessage::Quit, Show::PrintMessage),
    //             ["connect", user] => send_message(&write_stream, common::ClientMessage::Connect{name: String::from(*user)}, Show::PrintMessage),
    //             ["q"] => break,
    //             x => println!("unknown: {:?}", x),
    //         }
    //     }
    // }


    // println!("stopping client...");
    Ok(())
    



    // loop {
        
    //     let result = GameState::deserialize(&mut de);
    //     if let Ok(state) = result {
    //         println!("\nstate received:");
    //         println!("{:?}", state);

    //         let message = ClientMessage::Hello;
    //         let mut serialized = serde_json::to_vec(&message).unwrap();
    //         let _result = write_stream.write(&mut serialized);
    //         let _result = write_stream.write(&mut serialized);
    //         // let _result = write_stream.write(&mut serialized);
    //     } else {
    //         print!(".");
            
            
    //     }
    //     thread::sleep(time::Duration::from_millis(500));
    // }


    // let mymessage = common::ClientMessage::Connect {
    //     user_name: String::from("Lukas"),
    // };
    // let mut serialized = serde_json::to_vec(&mymessage).unwrap();
    // let _result = stream.write(&mut serialized);

    // loop {}



    // let mymessage = common::Message::Quit{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&mymessage).unwrap();
    // let _result = stream.write(&mut serialized);

    // let args: Vec<String> = env::args().collect();

    // if args.len() != 2 {
    //     panic!("usage: cmd [filename.yaml]");
    // }

    // let user_actions = testing::read_user_actions(&args[1]).unwrap();

    // for message_and_duration in user_actions.iter() {
    //     println!("{:?}", message_and_duration.message);
    //     println!("duration: {}s", message_and_duration.duration);
    // }
}
