use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode};
// use crossterm::style::{style, Color, Attribute};
use crossterm::style::{style, Attribute};


use crate::state;
use common::VoteState;

//press space to go to next phase
// FIXME show "ja/nein" above voter names

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
        crate::render::display_player_names(&shared, self.player_id.clone());
        crate::render::display_policy_cards(&shared);
        
        let left_margin = 25;
        

        let vote_selection = match (self.voted, self.selected_vote.clone()) {
            (false, VoteState::Ja) =>   Print("<Ja!>   Nein!   "),
            (false, VoteState::Nein) => Print(" Ja!   <Nein!> "),
            (true, VoteState::Ja) =>    Print("JA!"),
            (true, VoteState::Nein) =>  Print("NEIN!"),
        };

        let todo_str = match self.voted {
            true => Print("I voted"),
            false => Print("Vote..."),
        };

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin, 1),
            Print(style("Election").attribute(Attribute::Bold)),
            cursor::MoveTo(left_margin, 3),
            Print("Nominees"),
            cursor::MoveTo(left_margin, 5),
            Print(format!("President: {}", self.last_president.clone().unwrap() )),
            cursor::MoveTo(left_margin, 6),
            Print(format!("chancellor: {}", self.last_chancellor.clone().unwrap() )),
            cursor::MoveTo(left_margin, 8),
            todo_str,
            cursor::MoveTo(left_margin + 8, 8),
            vote_selection,
        );


        let number_of_votes = shared.players.iter().filter(|player| player.vote != None).count();
        let number_of_players = shared.players.len();

        let mut i_ja = 3;
        let mut i_nein = 3;
        let min_width = match number_of_players {
            10 => 2,
            _ => 1
        };
        let vote_complete = number_of_votes == number_of_players;

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin + 15 + 5, 1 as u16),
            Print(format!("Votes ({:width$}/{:width$})", number_of_votes, number_of_players, width = min_width)),
        );

        if vote_complete {
            // let _res = queue!(
            //     stdout(),
            //     cursor::MoveTo(left_margin + 8, 8),
            //     Print(format!("Ja!")),
            //     cursor::MoveTo(left_margin + 8, 8),
            //     Print(format!("Nein!")),
            // );
    
            for player in shared.players.clone() {
                match player.vote {
                    Some(VoteState::Ja) => {
                        let _res = queue!(
                            stdout(),
                            cursor::MoveTo(left_margin+15 + 5, i_ja as u16),
                            Print(format!("{}", player.player_id)),
                        );
                        i_ja += 1;
                    },
                    Some(VoteState::Nein) => {
                        let _res = queue!(
                            stdout(),
                            cursor::MoveTo(left_margin + 32,i_nein as u16),
                            Print(format!("{}", player.player_id)),
                        );
                        i_nein += 1;
                    },
                    None => {},
                }
                
            }
        }

        


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