use std::io::{stdout, Write};
use crossterm::{event, queue, cursor};
use crossterm::style::{Print};
use crossterm::event::{KeyEvent, KeyCode}; //, KeyCode};
use crossterm::style::{style, Attribute, Color};


use crate::state;
use common::PolicyCard;
use common::LegisationSubState;

#[derive(PartialEq, Clone, Debug)]
pub struct LegislativeSessionHandler {
    pub player_id: String,
    // pub ready: bool,
    pub president: String,
    pub chancellor: String,
    pub cursor_position: i16,
    pub substate: LegisationSubState,
    pub my_cards: Vec<PolicyCard>,
    pub selected_policies: Vec<bool>,
    pub ready: bool,
}


impl LegislativeSessionHandler {
    pub fn new(player_id: String, president: String, chancellor: String, substate: LegisationSubState,
            my_cards: Vec<PolicyCard>, cursor_position: i16, selected_policies: Vec<bool>, ready: bool) -> Self {
        Self {
            player_id,
            // ready: false,
            president,
            chancellor,
            cursor_position,
            substate,
            my_cards,
            selected_policies,
            ready,
        }
    }
}

impl state::ActionHandler for LegislativeSessionHandler {
    fn draw(&mut self, shared: &mut state::SharedState) {
        let left_margin = 25;

        // prototyping graphics
        // let mut cards : Vec<PolicyCard> = Vec::new();

        // cards.push(PolicyCard::Fascist);
        // cards.push(PolicyCard::Liberal);
        // cards.push(PolicyCard::Fascist);

        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin,1),
            Print(style("Legislative Session").attribute(Attribute::Bold)),
            cursor::MoveTo(left_margin,3),
            Print(format!("President: {:8}", self.president)),
            cursor::MoveTo(left_margin + 20,3),
            Print(format!("Chancellor: {:8}", self.chancellor)),
        );

        if shared.is_active(&self.player_id) {
            if self.player_id == self.president && self.my_cards.len() == 3 && self.substate == common::LegisationSubState::PresidentsChoice {

                for (i,card) in self.my_cards.iter().enumerate() {
                    let policy_letter = match (self.selected_policies[i], card) {
                        (true, PolicyCard::Liberal) => style("L").attribute(Attribute::Bold).with(Color::Blue),
                        (true, PolicyCard::Fascist) => style("F").attribute(Attribute::Bold).with(Color::Red),
                        (false, PolicyCard::Liberal) => style("L").attribute(Attribute::Bold),
                        (false, PolicyCard::Fascist) => style("F").attribute(Attribute::Bold),
                    };

                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin + i as u16 * 6, 5),Print(" ___ "), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 6),Print("|   |"), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 7),Print("|   |"),
                        cursor::MoveTo(left_margin + i as u16 * 6, 8),Print("|___|"), 
                        cursor::MoveTo(left_margin + i as u16 * 6 + 2, 7),Print(policy_letter),
                    );

                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin + self.cursor_position as u16 * 6 + 2, 10),Print("^"),
                    ); 
                }

                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin+20,6),
                    Print("Select 2 policies with space,"),
                    cursor::MoveTo(left_margin+20,7),
                    Print("confirm by pressing enter."),
                );
            }

            if self.player_id == self.chancellor && self.my_cards.len() == 2 && self.substate == common::LegisationSubState::ChancellorsChoice {

                for (i,card) in self.my_cards.iter().enumerate() {
                    let policy_letter = match (self.selected_policies[i], card) {
                        (true, PolicyCard::Liberal) => style("L").attribute(Attribute::Bold).with(Color::Blue),
                        (true, PolicyCard::Fascist) => style("F").attribute(Attribute::Bold).with(Color::Red),
                        (false, PolicyCard::Liberal) => style("L").attribute(Attribute::Bold),
                        (false, PolicyCard::Fascist) => style("F").attribute(Attribute::Bold),
                    };

                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin + i as u16 * 6, 5),Print(" ___ "), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 6),Print("|   |"), 
                        cursor::MoveTo(left_margin + i as u16 * 6, 7),Print("|   |"),
                        cursor::MoveTo(left_margin + i as u16 * 6, 8),Print("|___|"), 
                        cursor::MoveTo(left_margin + i as u16 * 6 + 2, 7),Print(policy_letter),
                    );

                    let _res = queue!(
                        stdout(),
                        cursor::MoveTo(left_margin + self.cursor_position as u16 * 6 + 2, 10),Print("^"),
                    ); 
                }

                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin+20,6),
                    Print("Press enter to enact policy."),
                );
            }
        

            if ((self.player_id == self.chancellor) || (self.player_id == self.president)) &&
                self.my_cards.len() == 1 &&
                self.substate == common::LegisationSubState::VetoPower {
                // veto power sub state
                
                
                let policy_letter = match self.my_cards[0] {
                    PolicyCard::Liberal => style("L").attribute(Attribute::Bold).with(Color::Blue),
                    PolicyCard::Fascist => style("F").attribute(Attribute::Bold).with(Color::Red),
                };

                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin + 6, 5),Print(" ___ "), 
                    cursor::MoveTo(left_margin + 6, 6),Print("|   |"), 
                    cursor::MoveTo(left_margin + 6, 7),Print("|   |"),
                    cursor::MoveTo(left_margin + 6, 8),Print("|___|"), 
                    cursor::MoveTo(left_margin + 6 + 2, 7),Print(policy_letter),
                );

                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin + 30, 5),
                    Print("Veto Power Unlocked (5 fascist policies enacted)"),
                    cursor::MoveTo(left_margin + 30, 7),
                    Print("If you want to veto the policy above agree inside"),
                    cursor::MoveTo(left_margin + 30, 8),
                    Print("the government."),
                    cursor::MoveTo(left_margin + 30, 9),
                    Print("Press [V] to veto"),
                    cursor::MoveTo(left_margin + 30, 10),
                    Print("Press [P] to pass"),
                );           

            } 
        }

        if self.substate == common::LegisationSubState::Done {
            // style("Liberal").attribute(Attribute::Bold).with(Color::Blue)
            // style("Fascist").attribute(Attribute::Bold).with(Color::Blue)
            if self.my_cards.len() > 0 {
                let polcy_type = match self.my_cards[0] {
                    PolicyCard::Liberal => style("Liberal").attribute(Attribute::Bold).with(Color::Blue),
                    PolicyCard::Fascist => style("Fascist").attribute(Attribute::Bold).with(Color::Red),
                };

                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin,6),
                    Print("A "),
                    Print(polcy_type),
                    Print(" policy was enacted."),
                );

                // FIXME this shoes policy was vetoed by the government for a short moment if 5 fascist and a liberal policy is passed
            } else if shared.fascist_policies_count == 5 {
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin,6),
                    Print("The policy was vetoed by the government."),
                );
            }

            if shared.is_active(&self.player_id) {
                    
                let ready_string = match self.ready {
                    true => String::from("[ready]"),
                    false => String::from("[press enter if ready]"),
                };
        
        
                let _res = queue!(
                    stdout(),
                    cursor::MoveTo(left_margin,8),
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

        if self.player_id == self.president && self.substate == common::LegisationSubState::PresidentsChoice {
            match event {
                KeyEvent{
                    code: KeyCode::Left,
                    modifiers: _,
                } => {
                    self.cursor_position = std::cmp::max(self.cursor_position - 1, 0);
                },
                KeyEvent{
                    code: KeyCode::Right,
                    modifiers: _,
                } => {
                    self.cursor_position = std::cmp::min(self.cursor_position + 1, 2);
                },
                KeyEvent{
                    code: KeyCode::Char(' '),
                    modifiers: _,
                } => {
                    let selected_count = self.selected_policies.iter().filter(|&p| *p).count();
                    if selected_count < 2 || self.selected_policies[self.cursor_position as usize] == true {
                        self.selected_policies[self.cursor_position as usize] = !self.selected_policies[self.cursor_position as usize];
                    }
                }
                
                KeyEvent{
                    code: KeyCode::Enter,
                    modifiers: _,
                } => {
                    let selected_count = self.selected_policies.iter().filter(|&p| *p).count();

                    if self.my_cards.len() == 3 && selected_count == 2 {
                        let mut policies: Vec<PolicyCard> = Vec::new();
                        for i in 0..3 {
                            if self.selected_policies[i] {
                                policies.push(self.my_cards[i].clone());
                            }
                        }
                        shared.outbox.push_back(common::ClientMessage::PolicyResponse{selected_policies: policies});
                    }
                    

                }
                _ => {},
            }
        }

        if self.player_id == self.chancellor && self.substate == common::LegisationSubState::ChancellorsChoice {
            match event {
                KeyEvent{
                    code: KeyCode::Left,
                    modifiers: _,
                } => {
                    self.cursor_position = std::cmp::max(self.cursor_position - 1, 0);
                },
                KeyEvent{
                    code: KeyCode::Right,
                    modifiers: _,
                } => {
                    self.cursor_position = std::cmp::min(self.cursor_position + 1, 1);
                },
                
                KeyEvent{
                    code: KeyCode::Enter,
                    modifiers: _,
                } => {
                    self.selected_policies[self.cursor_position as usize] = true;
                    let policies = vec![self.my_cards[self.cursor_position as usize].clone()];
                    shared.outbox.push_back(common::ClientMessage::PolicyResponse{selected_policies: policies});
                }
                _ => {},
            }
        }

        if (self.player_id == self.chancellor || self.player_id == self.president) && self.substate == common::LegisationSubState::VetoPower {
            match event {
                KeyEvent{
                    code: KeyCode::Char('v'),
                    modifiers: _,
                } => {
                    shared.outbox.push_back(common::ClientMessage::Vote{player_id: self.player_id.clone(), selected: common::VoteState::Nein});

                },
                KeyEvent{
                    code: KeyCode::Char('p'),
                    modifiers: _,
                } => {
                    shared.outbox.push_back(common::ClientMessage::Vote{player_id: self.player_id.clone(), selected: common::VoteState::Ja});
                },
                _ => {},
            }
        }
        if self.substate == common::LegisationSubState::Done {
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