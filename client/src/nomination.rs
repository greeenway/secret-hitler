use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::style::{style, Color};
use crossterm::event::{KeyEvent, KeyCode};

use crate::state;

#[derive(PartialEq, Clone, Debug)]
pub struct NominationHandler {
    pub player_id: String,
    pub presidential_nominee: String,
    is_president: bool,
    selected_index: Option<usize>,
    voted: bool,
}


impl NominationHandler {
    pub fn new(player_id: String, presidential_nominee: String) -> NominationHandler {
        let handler = Self {
            player_id: player_id.clone(),
            presidential_nominee: presidential_nominee.clone(),
            is_president: player_id == presidential_nominee,
            selected_index: None,
            voted: false,
        };

        handler
    }
}

impl state::ActionHandler for NominationHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        // TODO find a prettier solution to set the first selected player
        // not pretty to update this here but it is called regularily
        if self.is_president && self.selected_index == None {
            let my_player_index = shared.players.iter().position(|player| self.player_id == player.player_id).unwrap();
            if my_player_index == 0 {
                self.selected_index = Some(1);
            } else {
                self.selected_index = Some(0);
            }
        }



        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Nomination **"),
            cursor::MoveTo(1,8),
        );

        if self.is_president {
            let _res = queue!(
                stdout(),
                Print(format!("{}, you are presidential nominee, please select your chancellor: ", self.player_id)),
            );
            if let Some(selected_index) = self.selected_index {
                let mut draw_index = 0;
                for i in 0..shared.players.len() {
                    if shared.players[i].player_id != self.player_id {
                        let mut name = style(shared.players[i].player_id.clone());
                        if i == selected_index {
                            if !self.voted {
                                name = style(format!("{} <- ",shared.players[i].player_id.clone())).with(Color::Blue);
                            } else {
                                name = style(format!("{} is your nominee.",shared.players[i].player_id.clone())).with(Color::Green);
                            }
                            
                        }
                        draw_index += 1;
                        
                        let _res = queue!(
                            stdout(),
                            cursor::MoveTo(1,10 + draw_index as u16),
                            Print(name),
                        );
                    }
                }

            }
            
        } else {
            let _res = queue!(
                stdout(),
                Print(format!("Please wait while {} is nominating a chancellor candidate...", self.presidential_nominee)),
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
                if let Some(s) = self.selected_index {
                    self.voted = true;
                    let chancellor_nominee = shared.players[s].player_id.clone();
                    shared.outbox.push_back(common::ClientMessage::Nominated{chancellor_nominee: chancellor_nominee});
                    // TODO can we get stuck if this vote message gets lost? / reconnect?
                }
                
                // shared.outbox.push_back(common::ClientMessage::Ready{ready: self.ready});
            },
            KeyEvent{
                code: KeyCode::Up,
                modifiers: _,
            } => {
                if !self.voted {
                    if let Some(selected) = self.selected_index {
                        let my_player_index = shared.players.iter().position(|player| self.player_id == player.player_id).unwrap();
                        
                        let next_index = (selected + shared.players.len()) - 1; // make sure it is never negative
                        let mut next_index = next_index % shared.players.len();
                        if next_index == my_player_index {
                            next_index = ((next_index + shared.players.len()) - 1) % shared.players.len();
                        }

                        self.selected_index = Some(next_index);
                    }
                }
                
            },
            KeyEvent{
                code: KeyCode::Down,
                modifiers: _,
            } => {
                if !self.voted {
                    if let Some(selected) = self.selected_index {
                        let my_player_index = shared.players.iter().position(|player| self.player_id == player.player_id).unwrap();
                        let next_index = selected + 1;
                        let mut next_index = next_index % shared.players.len();
                        if next_index == my_player_index {
                            next_index += 1;
                            next_index = next_index % shared.players.len();
                        }
                        self.selected_index = Some(next_index);
                    }
                }

            }
            _ => {},
        }
    }
}
