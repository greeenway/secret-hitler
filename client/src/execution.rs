use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Color, Attribute};

use crate::state;
// use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
pub struct ExecutionHandler {
    pub ready: bool,
    pub player_id: String,
    pub president: String,
    pub selected_index: i16,
    pub executed: bool,
    pub victim: Option<String>,
}


impl ExecutionHandler {
    pub fn new(player_id: String, ready: bool, president: String, selected_index: i16, executed: bool, victim: Option<String>) -> ExecutionHandler {
        Self {
            player_id,
            ready,
            president,
            selected_index,
            executed,
            victim,
        }
    }
}


impl state::ActionHandler for ExecutionHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        let left_margin = 25;

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Executive Action - Execution").attribute(Attribute::Bold)),
        );

        if !self.executed {
            let players = shared.get_active_players();
            if self.player_id == self.president {
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin,3),
                    Print("A fascist policy was passed. You as the president"),
                    cursor::MoveTo(left_margin,4),
                    Print("can execute one player by pressing enter:"),
                );
                
                let mut j = 0;
                for i in 0..players.len() {
                    if players[i].player_id != self.player_id {
                        if self.selected_index == i as i16 {
                            let _res = queue!(
                                stdout(),
                                cursor::MoveTo(left_margin,6+j),
                                Print(style(format!("execute {}!", 
                                    players[i].player_id)).with(Color::Red))
                            );
                        } else {
                            let _res = queue!(
                                stdout(),
                                cursor::MoveTo(left_margin,6+j),
                                Print(format!("        {}!", 
                                    players[i].player_id))
                            );
                        }
                        j += 1;
                    }
                }
                
            } else {
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin,3),
                    Print("A fascist policy was passed. Please wait"),
                    cursor::MoveTo(left_margin,4),
                    Print(format!("until President {} executes a player.", self.president)),
                );
            }
        } else {

            if let Some(victim) = self.victim.clone() {
                if victim == self.player_id {
                    // you got executed
                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin,3),
                        Print(format!("President {} decided to execute you. You are dead.", self.president)),
                        cursor::MoveTo(left_margin,5),
                        Print("You can still watch the game but not participate."),
                        cursor::MoveTo(left_margin,6),
                        Print("By the rules you cannot talk about your identity else."),
                    );

                } else {
                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin,3),
                        Print(format!("President {} decided to execute {}.", self.president, victim)),
                        cursor::MoveTo(left_margin,5),
                        Print("RIP"),
                    );
                }
            }

            if shared.is_active(&self.player_id) {
                let ready_string = match self.ready {
                    true => String::from("[ready]"),
                    false => String::from("[press enter if ready]"),
                };
        
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin, 10),
                    Print(ready_string),
                );
            }
        }


        crate::render::display_player_names(&shared, self.player_id.clone());
        crate::render::display_policy_cards(&shared);
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        if !shared.is_active(&self.player_id) {
            return;
        }

        if !self.executed {
            let players = shared.get_active_players();
            if self.player_id == self.president {
                match event {
                KeyEvent{
                    code: KeyCode::Up,
                    modifiers: _,
                } => {
                    let my_player_index = players.iter().position(|player| self.player_id == player.player_id).unwrap();
                        
                    let next_index = (self.selected_index + players.len() as i16) - 1; // make sure it is never negative
                    let mut next_index = next_index % players.len() as i16;
                    if next_index == my_player_index as i16{
                        next_index = ((next_index + players.len() as i16) - 1) % players.len() as i16;
                    }

                    self.selected_index = next_index;
                },
                KeyEvent{
                    code: KeyCode::Down,
                    modifiers: _,
                } => {
                    let my_player_index = players.iter().position(|player| self.player_id == player.player_id).unwrap();
                    let next_index = self.selected_index + 1;
                    let mut next_index = next_index % players.len() as i16;
                    if next_index == my_player_index as i16 {
                        next_index += 1;
                        next_index = next_index % players.len() as i16;
                    }
                    self.selected_index = next_index;
                },
                KeyEvent{
                    code: KeyCode::Enter,
                    modifiers: _,
                } => {
                    shared.outbox.push_back(
                        common::ClientMessage::Execute{player_id: players[self.selected_index as usize].player_id.clone()}
                    );
                }
                _ => {},
            }
            }

            
        } else {
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
}