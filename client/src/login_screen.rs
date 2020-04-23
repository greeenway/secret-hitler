use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};

use crate::state;

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

impl state::ActionHandler for LoginScreenHandler {
    fn draw(&mut self, _: &mut state::SharedState) {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** LoginScreen **"),
            cursor::MoveTo(1,8),
            Print(format!("user: {}", self.input)),

        );
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
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