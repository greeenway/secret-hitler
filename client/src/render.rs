use std::io::{stdout, Write};
use crossterm::{queue, cursor};
use crossterm::style::{Print};

use crate::state;
use common::PartyMembership;
use common::ConnectionStatus;

pub fn display_player_names(shared: &state::SharedState) {
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
            (ConnectionStatus::Disconnected, _) => name_extensions.push(String::from("(disconnected)")),
        }

        let mut player_str = format!("{:14}", player.player_id);
        for ext in name_extensions {
            player_str = format!("{} {}", player_str, ext);
        }
        
        let _res = queue!(
            stdout(),
            cursor::MoveTo(1,15+rel_line as u16),
            Print(player_str)
        );
    }
}
