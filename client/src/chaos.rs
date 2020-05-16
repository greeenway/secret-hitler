use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Attribute};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct ChaosHandler {
    pub ready: bool,
    pub player_id: String,
}


impl ChaosHandler {
    pub fn new(player_id: String, ready: bool) -> ChaosHandler {
        Self {
            player_id,
            ready,
        }
    }
}


impl state::ActionHandler for ChaosHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        let left_margin = 25;

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Chaos").attribute(Attribute::Bold)),
            cursor::MoveTo(left_margin,3),
            Print("After three failed election chaos broke out."),
            cursor::MoveTo(left_margin,4),
            Print("The top policy card was enacted into law."),
        );

        
        if shared.is_active(&self.player_id) {
            let ready_string = match self.ready {
                true => String::from("[ready]"),
                false => String::from("[press enter if ready]"),
            };
    
    
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin, 6),
                Print(ready_string),
            );
        }
        


        crate::render::display_player_names(&shared, self.player_id.clone());
        crate::render::display_policy_cards(&shared);
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        if !shared.is_active(&self.player_id) {
            return;
        }
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