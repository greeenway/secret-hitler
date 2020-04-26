use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct NominationHandler {
    pub player_id: String,
    pub presidential_nominee: String,
    is_president: bool,
}


impl NominationHandler {
    pub fn new(player_id: String, presidential_nominee: String) -> NominationHandler {
        Self {
            player_id: player_id.clone(),
            presidential_nominee: presidential_nominee.clone(),
            is_president: player_id == presidential_nominee,
            
        }
    }
}

impl state::ActionHandler for NominationHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        // let mut user = String::from("- unknown -");
        // if let Some(user_name) = shared.user_name.clone() {
        //     user = user_name;
        // }
        let user = self.player_id.clone();

        // let ready_string = match self.ready {
        //     true => String::from("    [ready] "),
        //     false => String::from("    [Press Enter If Ready]    "),
        // };

        // let players_string = match shared.players.len() {
        //     1 => String::from("1 Player"),
        //     _ => format!("{} Players", shared.players.len())
        // };

        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Nomination **"),
            cursor::MoveTo(1,8),
        );

        if self.is_president {
            let _res = queue!(
                stdout(),
                Print("You are presidential nominee, please select your chancelor: "),
            );
        } else {
            let _res = queue!(
                stdout(),
                Print(format!("Please wait while {} is nominating a chancelor candidate...", self.presidential_nominee)),
            );
        }

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
