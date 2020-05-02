use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
// use crossterm::style::{style, Color, Attribute};


use crate::state;
// use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
pub struct LegislativeSessionHandler {
    pub player_id: String,
    // pub ready: bool,
    pub president: String,
    pub chancellor: String,
}


impl LegislativeSessionHandler {
    pub fn new(player_id: String, president: String, chancellor: String) -> Self {
        Self {
            player_id,
            // ready: false,
            president,
            chancellor
        }
    }
}

impl state::ActionHandler for LegislativeSessionHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
    


        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Legislative Session **"),
            cursor::MoveTo(0,9),
            Print(format!("President: {}", self.president)),
            cursor::MoveTo(0,10),
            Print(format!("Chancellor: {}", self.chancellor)),
        );


        crate::render::display_player_names(&shared);
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        match event {
            // KeyEvent{
            //     code: KeyCode::Enter,
            //     modifiers: _,
            // } => {
            //     self.ready = !self.ready;
            //     shared.outbox.push_back(common::ClientMessage::Ready{ready: self.ready});
            // }
            _ => {},
        }
    }
}