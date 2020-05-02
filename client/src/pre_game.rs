use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};
use crossterm::style::{style, Attribute};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct PreGameHandler {
    pub ready: bool,
    pub player_id: String,
}

// TODO disable joining of new users after pregame is completed
// TODO only advance once after pregame

impl PreGameHandler {
    pub fn new(player_id: String) -> PreGameHandler {
        Self {
            ready: false,
            player_id,
        }
    }
}

impl state::ActionHandler for PreGameHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        let left_margin = 25;
        let user = self.player_id.clone();

        let ready_string = match self.ready {
            true => String::from( "[ready]"),
            false => String::from("[press enter if ready]"),
        };


        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Pregame").attribute(Attribute::Bold)),
            cursor::MoveTo(left_margin,3),
            Print(format!("connected as {:8}", user)),
            cursor::MoveTo(left_margin,5),
            Print(ready_string),
        );

        crate::render::display_player_names(&shared, self.player_id.clone());
        crate::render::display_policy_cards(&shared);

    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        match event {
            KeyEvent{
                code: KeyCode::Enter,
                modifiers: _,
            } => {
                self.ready = !self.ready;
                shared.outbox.push_back(common::ClientMessage::Ready{ready: self.ready});
            }
            _ => {},
        }
    }
}
