use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode};
// use crossterm::style::{style, Color, Attribute};


use crate::state;
// use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
enum VoteState {
    Ja,
    Nein,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ElectionHandler {
    player_id: String,
    fail_count: u8,
    last_president: Option<String>,
    last_chancelor: Option<String>,
    selected_vote: VoteState,
}


impl ElectionHandler {
    pub fn new(player_id: String, fail_count: u8, last_president: Option<String>, last_chancelor: Option<String>) -> ElectionHandler {
        Self {
            player_id,
            fail_count,
            last_president,
            last_chancelor,
            selected_vote: VoteState::Ja,
        }
    }
}

impl state::ActionHandler for ElectionHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {


        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Election **"),

            cursor::MoveTo(1,9),
            Print("Nominees:"),
            cursor::MoveTo(1,10),
            Print(format!("President: {}", self.last_president.clone().unwrap() )),
            cursor::MoveTo(1,11),
            Print(format!("Chancelor: {}", self.last_chancelor.clone().unwrap() )),
            cursor::MoveTo(38,9),
            Print("Vote..."),
            cursor::MoveTo(37,11),
            Print("Ja!"),
            cursor::MoveTo(45,11),
            Print("Nein!"),
            
        );

        crate::render::display_player_names(&shared);
    }

    fn handle_event(&mut self, _: &mut state::SharedState, event: event::KeyEvent) {
        // first we need a president
        // then president needs to pick a chancelor
        // then players need to vote
        
        match event {

            _ => {},
        }
    }
}