use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Color, Attribute};


use crate::state;
use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
pub struct IdentityAssignmentHandler {
    pub player_id: String,
    pub ready: bool,
}


impl IdentityAssignmentHandler {
    pub fn new(player_id: String,) -> IdentityAssignmentHandler {
        Self {
            player_id,
            ready: false,
        }
    }
}

// FIXME can get stuck at awaiting identity...??

impl state::ActionHandler for IdentityAssignmentHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        
        // TODO should we catch this here or does this always work? -> now probably
        let my_player = shared.players.iter().find(|player| player.player_id == self.player_id).unwrap();  
        let mut assignment_ready = false;

        let mut party_membership_string = style("none");
        let mut hitler_string = style("");

        if let Some(membership) = my_player.party_membership.clone() {
            match (membership, my_player.is_hitler.unwrap()) {
                (PartyMembership::Fascist, true) => {
                    party_membership_string = style("Fascist").with(Color::Red);
                    hitler_string = style("HITLER")
                    .with(Color::Red)
                    .attribute(Attribute::Bold);
                },
                (PartyMembership::Fascist, false) => {
                    party_membership_string = style("Fascist").with(Color::Red);
                },
                (PartyMembership::Liberal, false) => {
                    party_membership_string = style("Liberal").with(Color::Blue);
                },
                _ => panic!("This should never happen: Hitler is a liberal..."),
               
            }
            assignment_ready = true;
        } 




        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Identity Assignment **"),
            cursor::MoveTo(0,9),
        );





        


        if assignment_ready {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,8),
                Print(format!("Hi {}, your party membership is ", self.player_id)),
                Print(party_membership_string),
                Print("."),
                cursor::MoveTo(1,9),
            );

            if my_player.is_hitler.unwrap() {
                let _res = queue!(
                    stdout(),
                    Print("You are the secret "),
                    Print(hitler_string),
                    Print("."),
                );
            }

            let ready_string = match self.ready {
                true => String::from("    [ready] "),
                false => String::from("    [Press Enter If Ready]    "),
            };
    
    
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,11),
                Print(ready_string),
            );
            



        } else {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,8),
                Print("Please await your identity..."),
            );
        }
        

        crate::render::display_player_names(&shared);
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