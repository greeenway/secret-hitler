use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};

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
        PreGameHandler {
            ready: false,
            player_id,
        }
    }
}

impl state::ActionHandler for PreGameHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        // let mut user = String::from("- unknown -");
        // if let Some(user_name) = shared.user_name.clone() {
        //     user = user_name;
        // }
        let user = self.player_id.clone();

        let ready_string = match self.ready {
            true => String::from("    [ready] "),
            false => String::from("    [Press Enter If Ready]    "),
        };

        let players_string = match shared.players.len() {
            1 => String::from("1 Player"),
            _ => format!("{} Players", shared.players.len())
        };

        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** PreGame **"),
            cursor::MoveTo(1,8),
            Print(format!("connected as {}", user)),
            cursor::MoveTo(1,10),
            Print(ready_string),
            cursor::MoveTo(1,13),
            Print(players_string),
        );

        crate::render::display_player_names(&shared);

        // for (rel_line, chat_message) in shared.chat_messages.iter().enumerate() {
        //     let _res = queue!(
        //         stdout(),
        //         cursor::MoveTo(1, 25 + rel_line as u16),
        //         Print(chat_message)
        //     );
        // }
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
