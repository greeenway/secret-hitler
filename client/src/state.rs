use std::collections::VecDeque;

use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};

use common::ServerMessage;

trait ActionHandler {
    fn draw(&mut self, shared: &mut SharedState);
    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent);
}

#[derive(PartialEq, Clone, Debug)]
pub struct LoginScreenHandler {
    input: String
}


impl LoginScreenHandler {
    pub fn new() -> LoginScreenHandler {
        LoginScreenHandler {
            input: String::from(""), // TODO add check for empty user name
        }
    }
}

impl ActionHandler for LoginScreenHandler {
    fn draw(&mut self, _: &mut SharedState) {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** LoginScreen **"),
            cursor::MoveTo(1,8),
            Print(format!("user: {}", self.input)),

        );
    }

    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent) {
        match event {
            KeyEvent{
                code: KeyCode::Char(c),
                modifiers: _,
            } => {
                self.input = format!("{}{}", self.input, c);
            }
            KeyEvent{
                code: KeyCode::Backspace,
                modifiers: _,
            } => {
                self.input.pop();
            },
            KeyEvent{
                code: KeyCode::Enter,
                modifiers: _,
            } => {
                shared.outbox.push_back(common::ClientMessage::Connect{name: self.input.clone()});
            },
            _ => {},
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PreGameHandler {
    
}

impl PreGameHandler {
    pub fn new() -> PreGameHandler {
        PreGameHandler {
            
        }
    }
}

impl ActionHandler for PreGameHandler {
    fn draw(&mut self, shared: &mut SharedState) {
        let mut user = String::from("- unknown -");
        if let Some(user_name) = shared.user_name.clone() {
            user = user_name;
        }

        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** PreGame **"),
            cursor::MoveTo(1,8),
            Print(format!("connected as {}", user)),
            cursor::MoveTo(1,10),
            Print("Players"),
        );

        for (rel_line, player) in shared.players.iter().enumerate() {
            let player_str = match player.connection_status {
                common::ConnectionStatus::Connected => player.player_id.clone(),
                common::ConnectionStatus::Disconnected => format!("{:12} (disconnected)", player.player_id),
            };
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,12+rel_line as u16),
                Print(player_str)
            );
        }

    }

    fn handle_event(&mut self, _: &mut SharedState, _: event::KeyEvent) {
        
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

    fn handle_event(&mut self, shared: &mut SharedState, event: event::KeyEvent) {
        match self {
            HandlerWrapper::LoginScreen(inner_handler) => inner_handler.handle_event(shared, event),
            HandlerWrapper::PreGame(inner_handler) => inner_handler.handle_event(shared, event),
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
            handler: HandlerWrapper::LoginScreen(LoginScreenHandler::new()),
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
                self.handler = HandlerWrapper::PreGame(PreGameHandler::new());
                self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::LoginScreen(_), ServerMessage::Connected{user_name}) => {
                self.handler = HandlerWrapper::PreGame(PreGameHandler::new());
                self.shared.user_name = Some(user_name);
            },
            (HandlerWrapper::PreGame(_), ServerMessage::StatusUpdate{players}) => {
                self.shared.players = players;
            },
            (_, ServerMessage::StatusUpdate{players: _}) => {
                // ignore for non pregame ... change later on
            },

            (state, message) => {
                panic!("unknown transition: {:?} / {:?}", state, message);
             } // remove this in the end
        }
    }
}


