use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Color, Attribute};

use crate::state;
use common::PolicyCard;

#[derive(PartialEq, Clone, Debug)]
pub struct PolicyPeekHandler {
    pub president: String,
    pub player_id: String,
    pub next_policies: Vec<PolicyCard>,
    pub ready: bool,
}


impl PolicyPeekHandler {
    pub fn new(player_id: String, president: String, next_policies: Vec<PolicyCard>, ready: bool) -> PolicyPeekHandler {
        Self {
            player_id,
            president,
            next_policies,
            ready,
        }
    }
}


impl state::ActionHandler for PolicyPeekHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        let left_margin = 25;

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Executive Action - Policy Peek").attribute(Attribute::Bold)),
        );

        if self.player_id == self.president {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin,3),
                Print("A fascist policy was passed. You as the president"),
                cursor::MoveTo(left_margin,4),
                Print("can see the next 3 cards in the polcy deck."),
            );

            if self.next_policies.len() == 3 {
                for (i,card) in self.next_policies.iter().enumerate() {
                    let policy_letter = match card {
                        PolicyCard::Liberal => style("L").attribute(Attribute::Bold).with(Color::Blue),
                        PolicyCard::Fascist => style("F").attribute(Attribute::Bold).with(Color::Red),
                    };

                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin + i as u16 * 6, 5),Print(" ___ "), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 6),Print("|   |"), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 7),Print("|   |"),
                        cursor::MoveTo(left_margin + i as u16 * 6, 8),Print("|___|"), 
                        cursor::MoveTo(left_margin + i as u16 * 6 + 2, 7),Print(policy_letter),
                    );

                }
                
            }

            let ready_string = match self.ready {
                true => String::from("[ready]"),
                false => String::from("[press enter if ready]"),
            };
    
    
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin, 10),
                Print(ready_string),
            );

        } else {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin,3),
                Print("A fascist policy was passed. Please wait"),
                cursor::MoveTo(left_margin,4),
                Print("until the president peeked at the next 3 policies."),
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
                if self.player_id == self.president {
                    self.ready = !self.ready;
                    shared.outbox.push_back(common::ClientMessage::Ready{ready: self.ready});
                }
            }
            _ => {},
        }
    }
}