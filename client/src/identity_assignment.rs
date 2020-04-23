use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct IdentityAssignmentHandler {
}


impl IdentityAssignmentHandler {
    pub fn new() -> IdentityAssignmentHandler {
        IdentityAssignmentHandler {
        }
    }
}

impl state::ActionHandler for IdentityAssignmentHandler {
    fn draw(&mut self, _: &mut state::SharedState) {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Identity Assignment **"),
        );
    }

    fn handle_event(&mut self, _: &mut state::SharedState, event: event::KeyEvent) {
        match event {
            // KeyEvent{
            //     code: KeyCode::Char(c),
            //     modifiers: _,
            // } => {
            //     self.input = format!("{}{}", self.input, c);
            // }
            // KeyEvent{
            //     code: KeyCode::Backspace,
            //     modifiers: _,
            // } => {
            //     self.input.pop();
            // },
            // KeyEvent{
            //     code: KeyCode::Enter,
            //     modifiers: _,
            // } => {
            //     shared.outbox.push_back(common::ClientMessage::Connect{name: self.input.clone()});
            // },
            _ => {},
        }
    }
}