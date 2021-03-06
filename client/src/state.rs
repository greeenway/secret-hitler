use std::collections::VecDeque;

use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};

use common::ServerMessage;
use common::ServerState;

use crate::login_screen;
use crate::pre_game;
use crate::identity_assignment;
use crate::election;
use crate::nomination;
use crate::legislative_session;
use crate::game_over;
use crate::policy_peek;
use crate::chaos;
use crate::execution;

pub trait ActionHandler {
    fn draw(&mut self, shared: &mut SharedState);
    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent);
}
#[derive(PartialEq, Clone, Debug)]
pub enum HandlerWrapper {
    LoginScreen(login_screen::LoginScreenHandler),
    PreGame(pre_game::PreGameHandler),
    IdentityAssignment(identity_assignment::IdentityAssignmentHandler),
    Nomination(nomination::NominationHandler),
    Election(election::ElectionHandler),
    LegislativeSession(legislative_session::LegislativeSessionHandler),
    GameOver(game_over::GameOverHandler),
    PolicyPeek(policy_peek::PolicyPeekHandler),
    Chaos(chaos::ChaosHandler),
    Execution(execution::ExecutionHandler),

}

impl ActionHandler for HandlerWrapper {
    fn draw(&mut self, shared: &mut SharedState) {
        match (self, shared.in_sync) {
            (HandlerWrapper::LoginScreen(inner_handler), _) => inner_handler.draw(shared),
            (HandlerWrapper::PreGame(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::IdentityAssignment(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Nomination(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Election(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::LegislativeSession(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::GameOver(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::PolicyPeek(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Chaos(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Execution(inner_handler), true) => inner_handler.draw(shared),
            _ => {
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(20,10),
                    Print("Reconnecting... "),
                );
            },
        }
    }

    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent) {
        match (self, shared.in_sync) {
            (HandlerWrapper::LoginScreen(inner_handler), _) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::PreGame(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::IdentityAssignment(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Nomination(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Election(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::LegislativeSession(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::GameOver(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::PolicyPeek(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Chaos(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Execution(inner_handler), true) => inner_handler.handle_event(shared, event),
            _ => {},
        }
    }
}

#[derive(Debug)]
pub struct SharedState {
    pub cmd_prompt: bool,
    pub enable_debug_console: bool,
    pub input: String,
    pub output: VecDeque<String>,
    pub done: bool,
    pub max_cmd_lines: usize,
    pub inbox: VecDeque<common::ServerMessage>,
    pub outbox: VecDeque<common::ClientMessage>,
    // pub user_name: Option<String>,
    pub players: Vec<common::Player>,
    pub chat_messages: VecDeque<String>,
    pub in_sync: bool,
    pub liberal_policies_count: u8,
    pub fascist_policies_count: u8,
}


impl SharedState {
    pub fn new(config: common::Configuration) -> SharedState {

        SharedState {
            cmd_prompt: false,
            enable_debug_console: config.enable_debug_console,
            input: String::from(""),
            output: VecDeque::new(),
            done: false,
            max_cmd_lines: 5,
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
            players: Vec::new(),
            chat_messages: VecDeque::new(),
            in_sync: true,
            liberal_policies_count: 0,
            fascist_policies_count: 0,
        }
    }

    pub fn get_active_players(&self) -> Vec<common::Player> {
        self.players.iter().filter(|p| p.status == common::PlayerState::Alive).cloned().collect()
    }

    pub fn get_players(&self) -> Vec<common::Player> {
        self.players.iter().filter(|p| p.status == common::PlayerState::Alive
            || p.status == common::PlayerState::Dead).cloned().collect()
    }

    pub fn get_observers(&self) -> Vec<common::Player> {
        self.players.iter().filter(|p| p.status == common::PlayerState::Observer).cloned().collect()
    }


    pub fn get_player_state(&self, player_id: &String) -> Option<common::PlayerState>{
        if let Some(player) = self.players.iter().find(|player| player.player_id == *player_id) {
            Some(player.status.clone())
        } else {
            None
        }
    }

    pub fn is_active(&self, player_id: &String) -> bool {
        match self.get_player_state(player_id) {
            Some(common::PlayerState::Alive) => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub handler: HandlerWrapper,
    pub shared: SharedState,
}

impl State {
    pub fn new(config: common::Configuration) -> State {
        State {
            handler: HandlerWrapper::LoginScreen(login_screen::LoginScreenHandler::new()),
            shared: SharedState::new(config),
        }
    }

    pub fn draw(&mut self) {
        self.handler.draw(&mut self.shared);
        // write here resyncing or something
    }

    pub fn handle_events(&mut self, event: event::KeyEvent) {
        self.handler.handle_event(&mut self.shared, event);
    }

    pub fn advance_handler(&mut self, message: ServerMessage) {

        match (self.handler.clone(), message) {
        
            (HandlerWrapper::LoginScreen(_), ServerMessage::Reconnected{user_name, state}) => {
                match state {
                    ServerState::Pregame => {
                        self.handler = HandlerWrapper::PreGame(pre_game::PreGameHandler::new(user_name));
                    },
                    ServerState::IdentityAssignment {identities_assigned: _} => {
                        self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new(user_name));
                    },
                    ServerState::Nomination {last_president: _, last_chancellor: _, presidential_nominee} => {
                        self.handler = HandlerWrapper::Nomination(nomination::NominationHandler::new(user_name, presidential_nominee));
                    },
                    ServerState::Election {election_count, presidential_nominee, chancellor_nominee} => {
                        self.handler = HandlerWrapper::Election(election::ElectionHandler::new(
                            user_name,
                            election_count,
                            Some(presidential_nominee),
                            Some(chancellor_nominee),
                        ))
                    },
                    ServerState::LegislativeSession{president, chancellor, substate, waiting: _} => {
                        let selected_policies = match substate {
                            common::LegisationSubState::PresidentsChoice => vec![false, false, false],
                            common::LegisationSubState::ChancellorsChoice => vec![false, false],
                            _ => Vec::new(),
                        };
                        self.handler = HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler::new(
                            user_name,
                            president,
                            chancellor,
                            substate,
                            Vec::new(),
                            0,
                            selected_policies,
                            false,
                        ))
                    },
                    ServerState::GameOver{winner} => {
                        self.handler = HandlerWrapper::GameOver(game_over::GameOverHandler::new(
                           user_name, winner))
                    },
                    ServerState::PolicyPeek{president, chancellor: _} => {
                        self.handler = HandlerWrapper::PolicyPeek(policy_peek::PolicyPeekHandler::new(
                            user_name, president, Vec::new(), false
                        ))
                    },
                    ServerState::Chaos{waiting: _, presidential_nominee: _} => {
                        self.handler = HandlerWrapper::Chaos(chaos::ChaosHandler::new(
                            user_name, false
                        ))
                    },
                    ServerState::Execution{executed, president, victim, chancellor: _} => {
                        self.handler = HandlerWrapper::Execution(execution::ExecutionHandler::new(
                            user_name, false, president, 0, executed, victim
                        ))
                    }
                    // _ => println!("unknown state!") //panic!("Reconnect to unknown state {:?}", state),
                }
                self.shared.in_sync = false;
                
                // self.shared.user_name = Some(user_name);
            },
            // old transitions ---> start
            (HandlerWrapper::LoginScreen(_), ServerMessage::Connected{user_name}) => {
                self.handler = HandlerWrapper::PreGame(pre_game::PreGameHandler::new(user_name));
                // self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::PreGame(pre_game::PreGameHandler{ready:_, player_id}), ServerMessage::Advance) => {
                self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new(player_id));
            },
            (HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler{player_id, ready: _}), 
                ServerMessage::AdvanceNomination{presidential_nominee}) => {
                self.handler = HandlerWrapper::Nomination(nomination::NominationHandler::new(player_id, presidential_nominee));
            },
            // end ---> old transitions

            // TODO clean up this file
            // (HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler{player_id, ready: _}), ServerMessage::Advance) => {
            //     self.handler = HandlerWrapper::Election(election::ElectionHandler::new(player_id, 0, None, None));
            // },
            (HandlerWrapper::PreGame(_), ServerMessage::Chat{user_name, message}) => {
                self.shared.chat_messages.push_back(format!("{}: {}", user_name, message));
            },
            (_, ServerMessage::Advance) => {}, // TODO is this still needed? or can we remove this?

            (HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler{player_id, president,
                chancellor, substate, my_cards: _, cursor_position, selected_policies, ready}),
                ServerMessage::PolicyUpdate{cards}) => {
                self.handler = HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler::new(
                    player_id,
                    president,
                    chancellor,
                    substate,
                    cards,
                    cursor_position,
                    selected_policies,
                    ready,
                ));

            },
            (HandlerWrapper::PolicyPeek(policy_peek::PolicyPeekHandler{president, player_id, next_policies: _, ready}) , ServerMessage::PolicyUpdate{cards}) => {
                self.handler = HandlerWrapper::PolicyPeek(policy_peek::PolicyPeekHandler::new(
                    player_id, president, cards, ready
                ));
            }, 
            (_, ServerMessage::PolicyUpdate{cards: _}) => {},
            // (HandlerWrapper::IdentityAssignment(_), ServerMessage::StatusUpdate{players}) => {
            //     self.shared.players = players;
            // },
            // idea: match server_state and current client state
            // if they are the same, stay in state, else move to next state as indicated by server
            // we potentially need to adjust the client states a bit to be robust against this frequent "resetting"
            (_, ServerMessage::StatusUpdate{players, state: server_state, player_id, 
                liberal_policies_count, fascist_policies_count}) => {
                self.shared.players = players;
                self.shared.in_sync = true;
                self.shared.liberal_policies_count = liberal_policies_count;
                self.shared.fascist_policies_count = fascist_policies_count;

                match (self.handler.clone(), server_state) {
                    // identity mappings
                    (HandlerWrapper::LoginScreen(_), _) => {}, // we need an explicit reconnect message to move to another state
                    (HandlerWrapper::PreGame(_), ServerState::Pregame) => {},
                    (HandlerWrapper::IdentityAssignment(_), ServerState::IdentityAssignment{identities_assigned: _}) => {},
                    (HandlerWrapper::Nomination(_), ServerState::Nomination{last_president: _, last_chancellor: _, presidential_nominee: _}) => {},
                    (HandlerWrapper::Election(_), ServerState::Election{election_count: _, chancellor_nominee: _, presidential_nominee: _}) => {},
                    (HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler{player_id, president,
                        chancellor, substate, my_cards: _, cursor_position: _, selected_policies: _, ready: _}), 
                        ServerState::LegislativeSession{president: _, chancellor: _, substate: s_substate, waiting: _}) => {
                            if substate != s_substate { // substate change
                                let selected_policies = match s_substate {
                                    common::LegisationSubState::PresidentsChoice => vec![false, false, false],
                                    common::LegisationSubState::ChancellorsChoice => vec![false, false],
                                    _ => Vec::new(),
                                };

                                self.handler = HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler::new(
                                    player_id,
                                    president,
                                    chancellor,
                                    s_substate,
                                    Vec::new(),
                                    0,
                                    selected_policies,
                                    false,
                                ));
                            }
                        }, 
                    (HandlerWrapper::GameOver(_), ServerState::GameOver{winner: _}) => {},
                    (HandlerWrapper::PolicyPeek(_), ServerState::PolicyPeek{president: _ , chancellor: _}) => {},
                    (HandlerWrapper::Chaos(_), ServerState::Chaos{waiting: _, presidential_nominee: _}) => {},
                    (HandlerWrapper::Execution(execution::ExecutionHandler{ready: _, player_id, selected_index: _, victim: _, president:_, executed}), 
                        ServerState::Execution{executed: s_executed, president: s_president, victim: s_victim, chancellor: _}) => {
                        if executed != s_executed {
                            self.handler = HandlerWrapper::Execution(execution::ExecutionHandler::new(
                                player_id, false, s_president, 0, s_executed, s_victim
                            ));
                        }
                    },
                        // actual state changes, not restricted, we trust that the server knows what it does
                    (_, ServerState::Pregame) => {
                        self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new(player_id.unwrap()));
                    },
                    (_, ServerState::IdentityAssignment{identities_assigned: _}) => {
                        self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new(player_id.unwrap()));
                        // todo use identities assigned or not?
                    },
                    (_, ServerState::Nomination{last_president: _, last_chancellor: _, presidential_nominee}) => {
                        self.handler = HandlerWrapper::Nomination(nomination::NominationHandler::new(player_id.unwrap(), presidential_nominee));
                    },
                    (_, ServerState::Election{election_count, chancellor_nominee, presidential_nominee}) => {
                        self.handler = HandlerWrapper::Election(election::ElectionHandler::new(
                            player_id.unwrap(),
                            election_count,
                            Some(presidential_nominee), 
                            Some(chancellor_nominee)
                        ));
                    },
                    (_, ServerState::LegislativeSession{president, chancellor, substate, waiting: _}) => {
                        self.handler = HandlerWrapper::LegislativeSession(legislative_session::LegislativeSessionHandler::new(
                            player_id.unwrap(),
                            president,
                            chancellor,
                            substate,
                            Vec::new(),
                            0,
                            vec![false, false, false],
                            false,
                        ));
                    },
                    (_, ServerState::GameOver{winner}) => {
                        self.handler = HandlerWrapper::GameOver(game_over::GameOverHandler::new(
                            player_id.unwrap(),
                            winner,
                        ));
                    },
                    (_, ServerState::PolicyPeek{president, chancellor: _}) => {
                        self.handler = HandlerWrapper::PolicyPeek(policy_peek::PolicyPeekHandler::new(
                            player_id.unwrap(), president, Vec::new(), false
                        ));
                    },
                    (_, ServerState::Chaos{waiting: _, presidential_nominee: _}) => {
                        self.handler = HandlerWrapper::Chaos(chaos::ChaosHandler::new(
                            player_id.unwrap(), false,
                        ));
                    },

                    (_, ServerState::Execution{executed, president, chancellor: _, victim}) => {
                        self.handler = HandlerWrapper::Execution(execution::ExecutionHandler::new(
                            player_id.unwrap(), false, president, 0, executed, victim
                        ));
                    },

                }
            },

            (state, message) => {
                panic!("unknown transition: {:?} / {:?}", state, message);
             } // remove this in the end
        }
    }
}


