use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
// use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Color, Attribute};

use crate::state;
use common::PartyMembership;

#[derive(PartialEq, Clone, Debug)]
pub struct GameOverHandler {
    pub winner: PartyMembership,
    pub player_id: String,
}


impl GameOverHandler {
    pub fn new(player_id: String, winner: PartyMembership) -> GameOverHandler {
        Self {
            player_id,
            winner,
        }
    }
}


impl state::ActionHandler for GameOverHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {

        let left_margin = 25;

        let winner_str = match self.winner {
            PartyMembership::Fascist => {style("Fascists").with(Color::Red)},
            PartyMembership::Liberal => {style("Liberals").with(Color::Blue)},
        };

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Game Over").attribute(Attribute::Bold)),
            cursor::MoveTo(left_margin,3),
            Print(winner_str),
            Print(" won!"),
        );

        let winner_color = match self.winner {
            PartyMembership::Fascist => Color::Red,
            PartyMembership::Liberal => Color::Blue,
        };
        

        let _res = queue!(
            stdout(),
            cursor::MoveTo(0,0), 
            Print(style("=".repeat(72)).with(winner_color)),
            cursor::MoveTo(0,22), 
            Print(style("=".repeat(72)).with(winner_color)),
        );

        if shared.fascist_policies_count < 6 && self.winner == PartyMembership::Fascist {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin, 5),
                Print("Hitler was elected as chancelor!"),
            );
        }

        if shared.liberal_policies_count < 5 && self.winner == PartyMembership::Liberal {
            let _res = queue!(
                stdout(),
                cursor::MoveTo(left_margin, 5),
                Print("Hitler was executed!"),
            );
        }

        crate::render::display_player_names(&shared, self.player_id.clone());
        crate::render::display_policy_cards(&shared);
    }

    fn handle_event(&mut self, shared: &mut state::SharedState, event: event::KeyEvent) {
        // match event {
        //     KeyEvent{
        //         code: KeyCode::Enter,
        //         modifiers: _,
        //     } => {
        //         self.ready = !self.ready;
        //         shared.outbox.push_back(common::ClientMessage::Ready{ready: self.ready});
        //     }
        //     _ => {},
        // }
    }
}