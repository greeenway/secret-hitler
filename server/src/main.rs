use std::io::prelude::*;

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::TcpListener;

#[macro_use]
extern crate machine;
extern crate rand;

#[derive(Clone, Debug, PartialEq)]
pub struct GameInfo {
    player_names: Vec<String>,
    player_count: usize,
    players_are_fascist: Vec<bool>,
    president_idx: usize,
    chancellor_idx: usize,
    policies_fascist_count: u8,
    policies_liberal_count: u8,
    round_count: u8,
}

machine!(
    enum GameState {
        PreGame { game_info: GameInfo },
        IdentityAssignment { game_info: GameInfo },
        Nomination { game_info: GameInfo },
        Election { game_info: GameInfo, fail_count: u8 },
        PolicySelectionPresident { game_info: GameInfo },
        PolicySelectionChancellor { game_info: GameInfo },
        Discussion { game_info: GameInfo },
        Chaos { game_info: GameInfo },
        RoundOver { game_info: GameInfo },
        GameOver { game_info: GameInfo },
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
      (Chaos, Advance) => Discussion,
      (PolicySelectionPresident, Advance) => PolicySelectionChancellor,
      (PolicySelectionChancellor, Advance) => Discussion,
      (Discussion, Advance) => RoundOver,
      (RoundOver, Advance) => [Nomination, GameOver]
    ]
  );

impl PreGame {
    pub fn on_advance(mut self, _: Advance) -> IdentityAssignment {
        self.game_info.player_count = self.game_info.player_names.len();

        println!("{:?}", self.game_info);

        IdentityAssignment {
            game_info: self.game_info,
        }
    }
}

impl IdentityAssignment {
    pub fn on_advance(mut self, _: Advance) -> Nomination {
        let mut rng = rand::thread_rng();
        // TODO: correct distribution!
        for _i in 0..self.game_info.player_count {
            self.game_info.players_are_fascist.push(rng.gen())
        }

        self.game_info.round_count = 1;
        self.game_info.president_idx = 0;

        println!("{:?}", self.game_info);

        Nomination {
            game_info: self.game_info,
        }
    }
}

impl Nomination {
    pub fn on_advance(self, _: Advance) -> Election {
        Election {
            game_info: self.game_info,
            fail_count: 0,
        }
    }
}

impl Election {
    pub fn on_advance(self, _: Advance) -> PolicySelectionPresident {
        PolicySelectionPresident {
            game_info: self.game_info,
        }
    }

    pub fn on_fail(self, _: Fail) -> GameState {
        let fail_count = self.fail_count + 1;
        if fail_count > 2 {
            GameState::chaos(self.game_info)
        } else {
            GameState::election(self.game_info, fail_count)
        }
    }
}

impl Chaos {
    pub fn on_advance(mut self, _: Advance) -> Discussion {
        // TODO: correct distribution
        let mut rng = rand::thread_rng();
        let is_fascist: bool = rng.gen();
        if is_fascist {
            self.game_info.policies_fascist_count += 1;
        } else {
            self.game_info.policies_liberal_count += 1;
        }

        Discussion {
            game_info: self.game_info,
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
            game_info: self.game_info,
        }
    }
}

impl PolicySelectionChancellor {
    pub fn on_advance(self, _: Advance) -> Discussion {
        Discussion {
            game_info: self.game_info,
        }
    }
}

impl Discussion {
    pub fn on_advance(self, _: Advance) -> RoundOver {
        RoundOver {
            game_info: self.game_info,
        }
    }
}

impl RoundOver {
    pub fn on_advance(mut self, _: Advance) -> GameState {
        if (self.game_info.policies_fascist_count == 6) || (self.game_info.policies_liberal_count == 5) {
            GameState::gameover(self.game_info)
        } else {
            self.game_info.round_count += 1;
            self.game_info.president_idx =
                (self.game_info.president_idx + 1) % self.game_info.player_count;
            GameState::nomination(self.game_info)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Connect { user_name: String },
    Quit { user_name: String },
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:34254").unwrap();

    // Initialize game info
    let mut game_info = GameInfo {
        player_names: vec![],
        player_count: 0,
        players_are_fascist: vec![],
        president_idx: 0,
        chancellor_idx: 0,
        policies_liberal_count: 0,
        policies_fascist_count: 0,
        round_count: 0,
    };

    game_info.player_names.push(String::from("Lukas"));
    game_info.player_names.push(String::from("Val"));
    game_info.player_names.push(String::from("Andi"));
    game_info.player_names.push(String::from("Tajna"));
    game_info.player_names.push(String::from("Stefan"));
    game_info.player_names.push(String::from("Marlene"));
    game_info.player_names.push(String::from("Markus"));

    // Initialize game state
    let mut game_state = GameState::PreGame(PreGame { game_info });

    // TODO: just testing
    game_state = game_state.on_advance(Advance);
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
