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

// FIXME can get stuck at awaiting identity...?? maybe connected with rejoin
// FIXME if player disconnects state can move because "all players are ready"

impl state::ActionHandler for IdentityAssignmentHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        let left_margin = 25;
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
            cursor::MoveTo(left_margin,1),
            Print(style("Identity Assignment").attribute(Attribute::Bold)),
        );



        


        if assignment_ready {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin,3),
                Print(format!("Hi {}, your party membership is ", self.player_id)),
                cursor::MoveTo(left_margin,4),
                Print(party_membership_string),
                Print("."),
                cursor::MoveTo(left_margin,5),
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
                true => String::from("[ready]"),
                false => String::from("[press enter if ready]"),
            };
    
    
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin,7),
                Print(ready_string),
            );
            



        } else {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin,3),
                Print("Please await your identity..."),
            );
        }
        

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