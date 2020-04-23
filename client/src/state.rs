use std::collections::VecDeque;

use crossterm::{event};
// use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode};

use common::ServerMessage;

use crate::login_screen;
use crate::pre_game;
use crate::identity_assignment;

pub trait ActionHandler {
    fn draw(&mut self, shared: &mut SharedState);
    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent);
}

#[derive(PartialEq, Clone, Debug)]
pub enum HandlerWrapper {
    LoginScreen(login_screen::LoginScreenHandler),
    PreGame(pre_game::PreGameHandler),
    IdentityAssignment(identity_assignment::IdentityAssignmentHandler),
}

impl ActionHandler for HandlerWrapper {
    fn draw(&mut self, shared: &mut SharedState) {
        match self {
            HandlerWrapper::LoginScreen(inner_handler) => inner_handler.draw(shared),
            HandlerWrapper::PreGame(inner_handler) => inner_handler.draw(shared),
            HandlerWrapper::IdentityAssignment(inner_handler) => inner_handler.draw(shared),
        }
    }

    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent) {
        match self {
            HandlerWrapper::LoginScreen(inner_handler) => inner_handler.handle_event(shared, event),
            HandlerWrapper::PreGame(inner_handler) => inner_handler.handle_event(shared, event),
            HandlerWrapper::IdentityAssignment(inner_handler) => inner_handler.handle_event(shared, event),
        }
    }
}

#[derive(Debug)]
pub struct SharedState {
    pub cmd_prompt: bool,
    pub input: String,
    pub output: VecDeque<String>,
    pub done: bool,
    pub max_cmd_lines: usize,
    pub inbox: VecDeque<common::ServerMessage>,
    pub outbox: VecDeque<common::ClientMessage>,
    pub user_name: Option<String>,
    pub players: Vec<common::Player>,
    pub chat_messages: VecDeque<String>,
}

impl SharedState {
    pub fn new() -> SharedState {
        SharedState {
            cmd_prompt: false,
            input: String::from(""),
            output: VecDeque::new(),
            done: false,
            max_cmd_lines: 5,
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
            user_name: None,
            players: Vec::new(),
            chat_messages: VecDeque::new(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub handler: HandlerWrapper,
    pub shared: SharedState,
}

impl State {
    pub fn new() -> State {
        State {
            handler: HandlerWrapper::LoginScreen(login_screen::LoginScreenHandler::new()),
            shared: SharedState::new(),
        }
    }

    pub fn draw(&mut self) {
        self.handler.draw(&mut self.shared);
    }

    pub fn handle_events(&mut self, event: event::KeyEvent) {
        self.handler.handle_event(&mut self.shared, event);
    }

    pub fn advance_handler(&mut self, message: ServerMessage) {
        match (self.handler.clone(), message) {
        
            (HandlerWrapper::LoginScreen(_), ServerMessage::Reconnected{user_name}) => {
                self.handler = HandlerWrapper::PreGame(pre_game::PreGameHandler::new());
                self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::LoginScreen(_), ServerMessage::Connected{user_name}) => {
                self.handler = HandlerWrapper::PreGame(pre_game::PreGameHandler::new());
                self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::PreGame(_), ServerMessage::StatusUpdate{players}) => {
                self.shared.players = players;
            },
            (HandlerWrapper::PreGame(_), ServerMessage::Advance) => {
                self.handler = HandlerWrapper::IdentityAssignment(identity_assignment::IdentityAssignmentHandler::new());
            },
            (HandlerWrapper::PreGame(_), ServerMessage::Chat{user_name, message}) => {
                self.shared.chat_messages.push_back(format!("{}: {}", user_name, message));
            },
            (_, ServerMessage::Advance) => {}, // TODO handle this more carefully
            (_, ServerMessage::StatusUpdate{players: _}) => {
                // ignore for non pregame ... change later on
            },

            (state, message) => {
                panic!("unknown transition: {:?} / {:?}", state, message);
             } // remove this in the end
        }
    }
}


