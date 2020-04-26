use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode};
use crossterm::style::{style, Color, Attribute};


use crate::state;
use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
pub struct IdentityAssignmentHandler {
    player_id: String,
}


impl IdentityAssignmentHandler {
    pub fn new(player_id: String,) -> IdentityAssignmentHandler {
        IdentityAssignmentHandler {
            player_id,
        }
    }
}

impl state::ActionHandler for IdentityAssignmentHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        // TODO should we catch this here or does this always work?
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
                    Print("Your are the secret "),
                    Print(hitler_string),
                    Print("."),
                );
            }
            



        } else {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,8),
                Print("Please await your identity..."),
            );
        }
        

        crate::render::display_player_names(&shared);
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