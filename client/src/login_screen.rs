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
        Self {
            input: String::from(""), // TODO add check for empty user name
        }
    }
}

impl state::ActionHandler for LoginScreenHandler {
    fn draw(&mut self, _: &mut state::SharedState) {
        let _res = queue!(
            stdout(),
            

            cursor::MoveTo(6,4), Print(" _____                    _     _   _ _ _   _        "),
            cursor::MoveTo(6,5), Print("/  ___|                  | |   | | | (_| | | |       "),   
            cursor::MoveTo(6,6), Print("\\ `--.  ___  ___ _ __ ___| |_  | |_| |_| |_| | ___ _ __ "),
            cursor::MoveTo(6,7), Print(" `--. \\/ _ \\/ __| '__/ _ | __| |  _  | | __| |/ _ | '__|"),
            cursor::MoveTo(6,8), Print("/\\__/ |  __| (__| | |  __| |_  | | | | | |_| |  __| |   "),
            cursor::MoveTo(6,9), Print("\\____/ \\___|\\___|_|  \\___|\\__| \\_| |_|_|\\__|_|\\___|_|  "),
            cursor::MoveTo(23,14),
            Print(format!("user name: {}", self.input)),
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