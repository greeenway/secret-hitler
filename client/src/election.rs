use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};
// use crossterm::style::{style, Color, Attribute};


use crate::state;
use common::VoteState;



#[derive(PartialEq, Clone, Debug)]
pub struct ElectionHandler {
    player_id: String,
    fail_count: u8,
    last_president: Option<String>,
    last_chancellor: Option<String>,
    selected_vote: VoteState,
    voted: bool,
}


impl ElectionHandler {
    pub fn new(player_id: String, fail_count: u8, last_president: Option<String>, last_chancellor: Option<String>) -> ElectionHandler {
        Self {
            player_id,
            fail_count,
            last_president,
            last_chancellor,
            selected_vote: VoteState::Ja,
            voted: false,
        }
    }
}

impl state::ActionHandler for ElectionHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        let vote_selection = match (self.voted, self.selected_vote.clone()) {
            (false, VoteState::Ja) =>   Print("<< Ja! >>     Nein!   "),
            (false, VoteState::Nein) => Print("   Ja!     << Nein! >>"),
            (true, VoteState::Ja) =>    Print("        JA!    "),
            (true, VoteState::Nein) =>  Print("       NEIN!    "),
        };

        let todo_str = match self.voted {
            true => Print("I voted"),
            false => Print("Vote..."),
        };

        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,7),
            Print("** Election **"),

            cursor::MoveTo(1,9),
            Print("Nominees:"),
            cursor::MoveTo(1,10),
            Print(format!("President: {}", self.last_president.clone().unwrap() )),
            cursor::MoveTo(1,11),
            Print(format!("chancellor: {}", self.last_chancellor.clone().unwrap() )),
            cursor::MoveTo(38,13),
            todo_str,
            cursor::MoveTo(37,14),
            vote_selection,
            
        );

        let mut i_ja = 5;
        let mut i_nein = 5;

        let _res = queue!(
            stdout(),
            cursor::MoveTo(35,2 as u16),
            Print(format!("---------- votes ----------")),
            cursor::MoveTo(35,3 as u16),
            Print(format!("Ja!")),
            cursor::MoveTo(52,3 as u16),
            Print(format!("Nein!")),
        );

        for player in shared.players.clone() {
            match player.vote {
                Some(VoteState::Ja) => {
                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(35,i_ja as u16),
                        Print(format!("{}", player.player_id)),
                    );
                    i_ja += 1;
                },
                Some(VoteState::Nein) => {
                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(52,i_nein as u16),
                        Print(format!("{}", player.player_id)),
                    );
                    i_nein += 1;
                },
                None => {},
            }
            
        }
        

        crate::render::display_player_names(&shared);
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        match event {
            KeyEvent{
                code: KeyCode::Enter,
                modifiers: _,
            } => {
                self.voted = true;
                shared.outbox.push_back(common::ClientMessage::Vote{selected: self.selected_vote.clone(), player_id: self.player_id.clone()});
            },
            KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
            } => {
                if !self.voted {
                    self.selected_vote = VoteState::Ja;
                }
            },
            KeyEvent {
                code: KeyCode::Right,
                modifiers: _,
            } => {
                if !self.voted {
                    self.selected_vote = VoteState::Nein;
                }
                
            },
            _ => {},
        }
    }
}