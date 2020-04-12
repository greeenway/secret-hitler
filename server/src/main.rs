use std::io::prelude::*;

use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::net::TcpListener;
use std::sync::Mutex;
use std::thread;

use std::env;

extern crate common;

#[macro_use]
extern crate machine;
extern crate rand;

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerIdentity {
    Liberal,
    Fascist,
    Hitler,
    Undefined,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    name: String,
    identity: PlayerIdentity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameInfo {
    players: Vec<Player>,
    player_count: usize,
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
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerJoin;

transitions!(GameState,
    [
      (PreGame, Advance) => IdentityAssignment,
      (PreGame, Player) => PreGame,
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

methods!(GameState,
    [
    PreGame => get game_info: GameInfo
    ]
  );

impl PreGame {
    pub fn on_advance(mut self, _: Advance) -> IdentityAssignment {
        self.game_info.player_count = self.game_info.players.len();

        println!("{:?}", self.game_info);

        IdentityAssignment {
            game_info: self.game_info,
        }
    }

    pub fn on_player(mut self, input: Player) -> PreGame {
        self.game_info.players.push(input);

        PreGame {
            game_info: self.game_info,
        }
    }
}

impl IdentityAssignment {
    pub fn on_advance(mut self, _: Advance) -> Nomination {
        /* # players | # liberals
            5           3
            6-7         4
            8-9         5
            10          6
        */

        // Initially set all to fascist
        for i in 0..self.game_info.player_count {
            self.game_info.players[i].identity = PlayerIdentity::Fascist;
        }

        // Set some to liberal and one to Hitler
        let liberalCount = match self.game_info.player_count {
            5 => 3,
            6 | 7 => 4,
            8 | 9 => 5,
            10 => 6,
            _ => panic!("Invalid player count"),
        };

        let mut rng = rand::thread_rng();

        let random_idxs =
            rand::seq::index::sample(&mut rng, self.game_info.player_count - 1, liberalCount + 1);

        for (i, idx) in random_idxs.iter().enumerate() {
            if i < liberalCount {
                self.game_info.players[idx].identity = PlayerIdentity::Liberal;
            } else {
                self.game_info.players[idx].identity = PlayerIdentity::Hitler;
            }
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
        if (self.game_info.policies_fascist_count == 6)
            || (self.game_info.policies_liberal_count == 5)
        {
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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: cmd [configfile.yaml]");
    }

    let config = common::Configuration::create_from_configfile(args[1].as_str()).unwrap();

    // Initialize game info
    let mut game_info = GameInfo {
        players: vec![],
        player_count: 0,
        president_idx: 0,
        chancellor_idx: 0,
        policies_liberal_count: 0,
        policies_fascist_count: 0,
        round_count: 0,
    };

    println!("{:?}", game_info);

    // Initialize game state
    // let mut game_state = GameState::PreGame(PreGame { game_info });
    let mut game_state_mutex = Mutex::new(GameState::PreGame(PreGame { game_info }));
    {
        let game_state = game_state_mutex.lock().unwrap();
        let info1 = game_state.game_info().clone();

        // advance
        println!("{:?}", info1);
    }

    // match game_state {
    //     GameState::PreGame => game_state.game_info(),
    //     _ => panic!("oh noes!"),
    // }

    // TODO: just testing
    // game_state = game_state.on_advance(Advance);
    // game_state = game_state.on_advance(Advance);

    let listener = TcpListener::bind(config.server_listen_address_and_port).unwrap();
    println!("listening started, ready to accept");

    thread::spawn(|| {
        // mutex!
        // apply game logic
        // debug: wait 1s
        // debug: print players joined
    });

    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = stream.unwrap();
            let mut de = serde_json::Deserializer::from_reader(stream);

            loop {
                let result = common::Message::deserialize(&mut de);

                if let Ok(message) = result {
                    println!("{:?}", message);

                    match message {
                        common::Message::Connect { user_name } => {
                            println!("user {} received!", user_name);

                            // let mut lock = game_state_mutex.try_lock();
                            // if let Ok(ref mut mutex) = lock {
                            //     mutex = mutex.on_player(Player {
                            //         name: user_name,
                            //         identity: PlayerIdentity::Undefined,
                            //     });
                            // } else {
                            //     println!("try_lock failed");
                            // }

                            {
                                let mut num = game_state_mutex.lock().unwrap();
                                *num = num.on_player(Player {
                                        name: user_name,
                                        identity: PlayerIdentity::Undefined,
                                    });
                            }

                            // let game_state = game_state_mutex.lock().unwrap();
                            // // let info1 = game_state.game_info().clone();
                            // let _a = game_state.on_player(Player {
                            //     name: user_name,
                            //     identity: PlayerIdentity::Undefined,
                            // });
                        }
                        _ => println!("something else was received!"),
                    }
                } else {
                    // println!("didn't get anything!");
                }

                // lock game state (mutex)
                // we need game state here
                // read action
                // big match case
                // depending on state
                // apply action to game state
                // send actions to clients
                //release mutex with {}
            }
        });
    }

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             println!("new client!");

    //             // let mut buffer: Vec<u8> = Vec::new();

    //             // // stream.write(&[1])?;
    //             // stream.read(&mut buffer)?;

    //             let mut de = serde_json::Deserializer::from_reader(stream);

    //             loop {
    //                 // let read_bytes = stream.peek(&mut [0, 1000]).unwrap();

    //                 let result = Message::deserialize(&mut de);

    //                 if let Ok(message) = result {
    //                     println!("{:?}", message);

    //                     match message {
    //                         Message::Connect { user_name } => {
    //                             println!("user {} received!", user_name)
    //                         }
    //                         _ => println!("something else was received!"),
    //                     }
    //                 } else {
    //                     // println!("didn't get anything!");
    //                 }

    //                 // match result {
    //                 //     Result::Message(message) => {
    //                 //         println!("{:?}", message);

    //                 //         match message {
    //                 //             Message::Connect{user_name} => println!("user {} received!", user_name),
    //                 //             _ => println!("something else was received!"),
    //                 //         }
    //                 //     },
    //                 //     // serde_json::error::Error
    //                 //     _ => {
    //                 //         println!("didn't get anything!");
    //                 //     }
    //                 // }
    //             }
    //         }
    //         Err(_e) => { /* connection failed */ }
    //     }
    // }

    Ok(())
}
