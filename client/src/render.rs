use std::io::{stdout, Write};
use crossterm::{queue, cursor};
use crossterm::style::{Print};
use crossterm::style::{style, Color, Attribute};

use crate::state;
use common::PartyMembership;
use common::ConnectionStatus;

pub fn display_player_names(shared: &state::SharedState, player_id: String) {
    for (rel_line, player) in shared.players.iter().enumerate() {

        let mut name_extensions: Vec<String>= Vec::new();
        
        // determine party membership string
        if let Some(membership) = player.party_membership.clone() {
            match (membership, player.is_hitler.unwrap()) {
                (PartyMembership::Fascist, true) => name_extensions.push(String::from("[H]")),
                (PartyMembership::Fascist, false) => name_extensions.push(String::from("[F]")),
                (PartyMembership::Liberal, false) => name_extensions.push(String::from("[L]")),
                _ => panic!("This should never happen: Hitler is a liberal..."),
            }
        }

        // determine connection status string
        match (player.connection_status.clone(), player.ready) {
            (ConnectionStatus::Connected, true) => name_extensions.push(String::from("(ready)")),
            (ConnectionStatus::Connected, false) => {},
            (ConnectionStatus::Disconnected, _) => name_extensions.push(String::from("(disc)")),
        }

        let mut player_str = format!("{:8}", player.player_id);
        for ext in name_extensions {
            player_str = format!("{} {}", player_str, ext);
        }
        
        let _res = queue!(
            stdout(),
            cursor::MoveTo(1,1 as u16),
            Print(style("Players").attribute(Attribute::Bold)),
            cursor::MoveTo(1,3+rel_line as u16),
            Print(player_str)
        );

        
    }
    if let Some(me) = shared.players.iter().find(|player| player.player_id == player_id) {
        let connect_str = match (&me.party_membership, me.is_hitler) {
            (Some(_), Some(_)) => style(", "),
            _ => style(""),
        };
        let role_str = match (&me.party_membership, me.is_hitler) {
            (Some(common::PartyMembership::Fascist), Some(true)) => style("Hitler").with(Color::Red),
            (Some(common::PartyMembership::Fascist), Some(false)) => style("Fascist").with(Color::Red),
            (Some(common::PartyMembership::Liberal), Some(false)) => style("Liberal").with(Color::Blue),
            _ => style(""),
        };
    
        let _res = queue!(
            stdout(),
            cursor::MoveTo(1, 20),
            Print(style(player_id).attribute(Attribute::Bold)),
            Print(connect_str),
            Print(role_str),
        );
    }
}

pub fn display_policy_cards(shared: &state::SharedState) {
    let left_margin = 25;
    let _res = queue!(
        stdout(),
        cursor::MoveTo(left_margin + 3,11),Print("  ___    ___    ___    ___    ___    ___"), 
        cursor::MoveTo(left_margin + 3,12),Print(" |   |  |   |  |   |  |   |  |   |  |   |"), 
        cursor::MoveTo(left_margin + 3,13),Print(" |   |  |   |  |   |  |   |  |   |  |   |"), 
        cursor::MoveTo(left_margin + 3,14),Print(" |___|  |___|  |___|  |___|  |___|  |___|"), 
        cursor::MoveTo(left_margin + 3,15),Print("                                           "), 
        cursor::MoveTo(left_margin + 3,16),Print("------------------------------------------ "),
        cursor::MoveTo(left_margin + 3,17),Print("  ___    ___    ___    ___    ___          "), 
        cursor::MoveTo(left_margin + 3,18),Print(" |   |  |   |  |   |  |   |  |   |         "), 
        cursor::MoveTo(left_margin + 3,19),Print(" |   |  |   |  |   |  |   |  |   |         "), 
        cursor::MoveTo(left_margin + 3,20),Print(" |___|  |___|  |___|  |___|  |___|         "), 
    );

    for i in 0..shared.fascist_policies_count {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin + 6 + (i as u16 *7),13), Print(style("F").attribute(Attribute::Bold).with(Color::Red)),
        );
    }

    for i in 0..shared.liberal_policies_count {
        let _res = queue!(
            stdout(),
            cursor::MoveTo(left_margin + 6 + (i as u16 *7),19), Print(style("L").attribute(Attribute::Bold).with(Color::Blue)),
        );
    }

}

