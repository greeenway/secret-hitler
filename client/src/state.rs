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
                    ServerState::Election {fail_count, presidential_nominee, chancellor_nominee} => {
                        self.handler = HandlerWrapper::Election(election::ElectionHandler::new(
                            user_name,
                            fail_count,
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
                    ServerState::GameOver => {},
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
                    (HandlerWrapper::Election(_), ServerState::Election{fail_count: _, chancellor_nominee: _, presidential_nominee: _}) => {},
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
                    (_, ServerState::Election{fail_count, chancellor_nominee, presidential_nominee}) => {
                        self.handler = HandlerWrapper::Election(election::ElectionHandler::new(
                            player_id.unwrap(),
                            fail_count,
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
                    // todo other state changes
                    (_, _) => {} // we want to switch states later
                }
            },

            (state, message) => {
                panic!("unknown transition: {:?} / {:?}", state, message);
             } // remove this in the end
        }
    }
}


