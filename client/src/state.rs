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
}

impl ActionHandler for HandlerWrapper {
    fn draw(&mut self, shared: &mut SharedState) {
        match (self, shared.in_sync) {
            (HandlerWrapper::LoginScreen(inner_handler), _) => inner_handler.draw(shared),
            (HandlerWrapper::PreGame(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::IdentityAssignment(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Nomination(inner_handler), true) => inner_handler.draw(shared),
            (HandlerWrapper::Election(inner_handler), true) => inner_handler.draw(shared),
            _ => {
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(3,8),
                    Print("Reconnecting... "),
                    
        
                );
            },
        }
    }

    // TODO also guard handle events like draw handler
    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent) {
        match (self, shared.in_sync) {
            (HandlerWrapper::LoginScreen(inner_handler), _) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::PreGame(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::IdentityAssignment(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Nomination(inner_handler), true) => inner_handler.handle_event(shared, event),
            (HandlerWrapper::Election(inner_handler), true) => inner_handler.handle_event(shared, event),
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
                    ServerState::Nomination {last_president: _, last_chancelor: _, presidential_nominee} => {
                        self.handler = HandlerWrapper::Nomination(nomination::NominationHandler::new(user_name, presidential_nominee));
                    },
                    // ServerState::Election {fail_count: _, presidential_nominee, chancelor_nominee} => {},
                    ServerState::GameOver => {},
                    _ => println!("unknown state!") //panic!("Reconnect to unknown state {:?}", state),
                }
                self.shared.in_sync = false;
                
                // self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::LoginScreen(_), ServerMessage::Connected{user_name}) => {
                self.handler = HandlerWrapper::PreGame(pre_game::PreGameHandler::new(user_name));
                // self.shared.user_name = Some(user_name);
            },
            // (HandlerWrapper::PreGame(_), ServerMessage::StatusUpdate{players}) => {
            //     self.shared.players = players;
            // },
            (HandlerWrapper::PreGame(pre_game::PreGameHandler{ready:_, player_id}), ServerMessage::Advance) => {
                self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new(player_id));
            },
            (HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler{player_id, ready: _}), 
                ServerMessage::AdvanceNomination{presidential_nominee}) => {
                self.handler = HandlerWrapper::Nomination(nomination::NominationHandler::new(player_id, presidential_nominee));
            },
            // (HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler{player_id, ready: _}), ServerMessage::Advance) => {
            //     self.handler = HandlerWrapper::Election(election::ElectionHandler::new(player_id, 0, None, None));
            // },
            (HandlerWrapper::PreGame(_), ServerMessage::Chat{user_name, message}) => {
                self.shared.chat_messages.push_back(format!("{}: {}", user_name, message));
            },
            (_, ServerMessage::Advance) => {}, // TODO handle this more carefully
            // (HandlerWrapper::IdentityAssignment(_), ServerMessage::StatusUpdate{players}) => {
            //     self.shared.players = players;
            // },
            (_, ServerMessage::StatusUpdate{players}) => {
                self.shared.players = players;
                self.shared.in_sync = true;
            },

            (state, message) => {
                panic!("unknown transition: {:?} / {:?}", state, message);
             } // remove this in the end
        }
    }
}


