use std::io::prelude::*;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::TcpListener;

#[macro_use]
extern crate machine;
extern crate rand;

machine!(
    enum GameState {
        PreGame,
        IdentityAssignment,
        Nomination,
        Election { fail_count: u8 },
        PolicySelectionPresident,
        PolicySelectionChancellor,
        Discussion,
        Chaos,
        GameOver,
        // TODO: execution and all executive action states
    }
);

#[derive(Clone, Debug, PartialEq)]
pub struct Advance;
#[derive(Clone, Debug, PartialEq)]
pub struct Fail;

transitions!(GameState,
    [
      (PreGame, Advance) => IdentityAssignment,
      (IdentityAssignment, Advance) => Nomination,
      (Nomination, Advance) => Election,
      (Election, Advance) => PolicySelectionPresident,
      (Election, Fail) => [Election, Chaos],
      (Chaos, Advance) => [Nomination, GameOver],
      (PolicySelectionPresident, Advance) => PolicySelectionChancellor,
      (PolicySelectionChancellor, Advance) => [Discussion, GameOver],
      (Discussion, Advance) => Nomination
    ]
  );

impl PreGame {
    pub fn on_advance(self, _: Advance) -> IdentityAssignment {
        IdentityAssignment {}
    }
}

impl IdentityAssignment {
    pub fn on_advance(self, _: Advance) -> Nomination {
        Nomination {}
    }
}

impl Nomination {
    pub fn on_advance(self, _: Advance) -> Election {
        Election { fail_count: 0 }
    }
}

impl Election {
    pub fn on_advance(self, _: Advance) -> PolicySelectionPresident {
        PolicySelectionPresident {}
    }

    pub fn on_fail(self, _: Fail) -> GameState {
        let fail_count = self.fail_count + 1;
        if fail_count > 2 {
            GameState::chaos()
        } else {
            GameState::election(fail_count)
        }
    }
}

impl Chaos {
    pub fn on_advance(self, _: Advance) -> GameState {
        // TODO: implement functionality
        let mut rng = rand::thread_rng();
        if rng.gen_range(0, 2) > 0 {
            GameState::gameover()
        } else {
            GameState::nomination()
        }
    }
}

// TODO: fix
impl PolicySelectionPresident {
    pub fn on_advance(self, _: Advance) -> PolicySelectionChancellor {
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(0, 4);
        PolicySelectionChancellor {
            // TODO: consider distribution of fascist/liberal policies
            //fascistPoliciesCount = rand_num
        }
    }
}

impl PolicySelectionChancellor {
    pub fn on_advance(self, _: Advance) -> GameState {
        GameState::discussion()
        // TODO: or gameover
    }
}

impl Discussion {
    pub fn on_advance(self, _: Advance) -> Nomination {
        Nomination {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Connect { user_name: String },
    Quit { user_name: String },
}

fn main() -> std::io::Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let listener = TcpListener::bind("127.0.0.1:34254").unwrap();

    // Initialize game state
    let mut game_state = GameState::PreGame(PreGame {});

    // TODO: just testing
    game_state = game_state.on_advance(Advance);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");

                // let mut buffer: Vec<u8> = Vec::new();

                // // stream.write(&[1])?;
                // stream.read(&mut buffer)?;

                let mut de = serde_json::Deserializer::from_reader(stream);

                loop {
                    // let read_bytes = stream.peek(&mut [0, 1000]).unwrap();

                    let result = Message::deserialize(&mut de);

                    if let Ok(message) = result {
                        println!("{:?}", message);

                        match message {
                            Message::Connect { user_name } => {
                                println!("user {} received!", user_name)
                            }
                            _ => println!("something else was received!"),
                        }
                    } else {
                        // println!("didn't get anything!");
                    }

                    // match result {
                    //     Result::Message(message) => {
                    //         println!("{:?}", message);

                    //         match message {
                    //             Message::Connect{user_name} => println!("user {} received!", user_name),
                    //             _ => println!("something else was received!"),
                    //         }
                    //     },
                    //     // serde_json::error::Error
                    //     _ => {
                    //         println!("didn't get anything!");
                    //     }
                    // }
                }
            }
            Err(_e) => { /* connection failed */ }
        }
    }

    Ok(())
}
