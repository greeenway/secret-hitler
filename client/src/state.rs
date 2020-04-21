use std::collections::VecDeque;

use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};

use common::ServerMessage;

trait ActionHandler {
    fn draw(&mut self, shared: &mut SharedState);
}

#[derive(PartialEq, Clone, Debug)]
struct LoginScreenHandler {
    input: String
}


impl LoginScreenHandler {
    pub fn new() -> LoginScreenHandler {
        LoginScreenHandler {
            input: String::from("username"),
        }
    }
}

impl ActionHandler for LoginScreenHandler {
    fn draw(&mut self, shared: &mut SharedState) {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** LoginScreen **"),
            cursor::MoveTo(1,8),
            Print(format!("user: {}", self.input)),

        );
    }
}

#[derive(PartialEq, Clone, Debug)]
struct PreGameHandler {
    
}

impl PreGameHandler {
    pub fn new() -> PreGameHandler {
        PreGameHandler {
            
        }
    }
}

impl ActionHandler for PreGameHandler {
    fn draw(&mut self, shared: &mut SharedState) {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** PreGame **"),
        );
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum HandlerWrapper {
    LoginScreen(LoginScreenHandler),
    PreGame(PreGameHandler),
}


impl ActionHandler for HandlerWrapper {
    fn draw(&mut self, shared: &mut SharedState) {
        match self {
            HandlerWrapper::LoginScreen(inner_handler) => inner_handler.draw(shared),
            HandlerWrapper::PreGame(inner_handler) => inner_handler.draw(shared),
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
        }
    }
}

pub enum Message {
    Stay {name: String},
    Change,
}

#[derive(Debug)]
pub struct State {
    pub handler: HandlerWrapper,
    pub shared: SharedState,
}

impl State {
    pub fn new() -> State {
        State {
            handler: HandlerWrapper::LoginScreen(LoginScreenHandler::new()),
            shared: SharedState::new(),
        }
    }

    pub fn draw(&mut self) {
        self.handler.draw(&mut self.shared);
    }

    pub fn handle_events(&mut self, event: event::KeyEvent) {

    }

    pub fn advance_handler(&mut self, message: ServerMessage) {
        match (self.handler.clone(), message) {
        
            (HandlerWrapper::LoginScreen(_), ServerMessage::Reconnected{user_name})  => self.handler = HandlerWrapper::PreGame(PreGameHandler::new()),
            // (HandlerWrapper::LoginScreen(_), Message::Change)  => self.handler = HandlerWrapper::PreGame(PreGameHandler::new()),
            // (HandlerWrapper::PreGame(_), Message::Stay{name: _})  => self.handler = HandlerWrapper::PreGame(PreGameHandler::new()),
            // (HandlerWrapper::PreGame(_), Message::Change)  => self.handler = HandlerWrapper::LoginScreen(LoginScreenHandler::new()),
            _ => panic!("unhandled transition") // remove this in the end
        }
    }
}