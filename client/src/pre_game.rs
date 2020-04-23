use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct PreGameHandler {
    ready: bool,
}

// TODO disable joining of new users after pregame is completed
// TODO only advance once after pregame

impl PreGameHandler {
    pub fn new() -> PreGameHandler {
        PreGameHandler {
            ready: false,
        }
    }
}

impl state::ActionHandler for PreGameHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        let mut user = String::from("- unknown -");
        if let Some(user_name) = shared.user_name.clone() {
            user = user_name;
        }

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

        for (rel_line, player) in shared.players.iter().enumerate() {
            let player_str = match player.connection_status {
                common::ConnectionStatus::Connected => {
                    if player.ready {
                        format!("{:14} (ready)", player.player_id)
                    } else {
                        player.player_id.clone()
                    }
                },
                common::ConnectionStatus::Disconnected => format!("{:14} (disconnected)", player.player_id),
            };
            let _res = queue!(
                stdout(),
                cursor::MoveTo(1,15+rel_line as u16),
                Print(player_str)
            );
        }

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