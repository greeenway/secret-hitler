// disabled for now
// use std::collections::{VecDeque};

// use common::{ConnectionStatus, ServerState, Player,
//     PartyMembership, Configuration};
// use crate::state::GameState;

// pub fn seed_game_state(config: Configuration, state: &str) -> GameState {
//     match state {
//         "nomination" => {
//             let mut game_state = GameState::new(config);
    
//             game_state.state = ServerState::IdentityAssignment{identities_assigned: false};
            
//             let mut p1 = Player::new(String::from("lukas"), 0);
//             p1.connection_status = ConnectionStatus::Disconnected;
//             p1.is_hitler = Some(false);
//             p1.party_membership = Some(PartyMembership::Liberal);
//             p1.ready = false;
        
//             let mut p2 = Player::new(String::from("val"), 1);
//             p2.connection_status = ConnectionStatus::Disconnected;
//             p2.is_hitler = Some(false);
//             p2.party_membership = Some(PartyMembership::Fascist);
//             p2.ready = false;
        
//             let mut p3 = Player::new(String::from("markus"), 2);
//             p3.connection_status = ConnectionStatus::Disconnected;
//             p3.is_hitler = Some(true);
//             p3.party_membership = Some(PartyMembership::Fascist);
//             p3.ready = false;
        
        
//             game_state.shared.players.push(p1);
//             game_state.shared.players.push(p2);
//             game_state.shared.players.push(p3);
        
//             game_state.shared.player_number = Some(3);
//             game_state.shared.fascist_known_by_hitler = Some(true);
//             game_state.shared.outboxes.insert(0, VecDeque::new());
//             game_state.shared.outboxes.insert(1, VecDeque::new());
//             game_state.shared.outboxes.insert(2, VecDeque::new());
//             game_state
//         },
//         "election" => {
//             let mut game_state = GameState::new(config);
    
//             game_state.state = ServerState::Election{election_count: 1,
//                 presidential_nominee: String::from("lukas"),
//                 chancellor_nominee: String::from("val"),
//             };
            
//             let mut p1 = Player::new(String::from("lukas"), 0);
//             p1.connection_status = ConnectionStatus::Disconnected;
//             p1.is_hitler = Some(false);
//             p1.party_membership = Some(PartyMembership::Liberal);
//             p1.ready = false;
        
//             let mut p2 = Player::new(String::from("val"), 1);
//             p2.connection_status = ConnectionStatus::Disconnected;
//             p2.is_hitler = Some(false);
//             p2.party_membership = Some(PartyMembership::Fascist);
//             p2.ready = false;
        
//             let mut p3 = Player::new(String::from("markus"), 2);
//             p3.connection_status = ConnectionStatus::Disconnected;
//             p3.is_hitler = Some(true);
//             p3.party_membership = Some(PartyMembership::Fascist);
//             p3.ready = false;
        
        
//             game_state.shared.players.push(p1);
//             game_state.shared.players.push(p2);
//             game_state.shared.players.push(p3);
        
//             game_state.shared.player_number = Some(3);
//             game_state.shared.fascist_known_by_hitler = Some(true);
//             game_state.shared.outboxes.insert(0, VecDeque::new());
//             game_state.shared.outboxes.insert(1, VecDeque::new());
//             game_state.shared.outboxes.insert(2, VecDeque::new());
//             game_state
//         },
//         "legislative_session" => {
//             let mut game_state = GameState::new(config);
    
//             game_state.state = ServerState::LegislativeSession{
//                 chancellor: String::from("lukas"),
//                 president: String::from("val"),
//                 substate: common::LegisationSubState::PresidentsChoice,
//                 waiting: false,
//             };
            
//             let mut p1 = Player::new(String::from("lukas"), 0);
//             p1.connection_status = ConnectionStatus::Disconnected;
//             p1.is_hitler = Some(false);
//             p1.party_membership = Some(PartyMembership::Liberal);
//             p1.ready = false;
        
//             let mut p2 = Player::new(String::from("val"), 1);
//             p2.connection_status = ConnectionStatus::Disconnected;
//             p2.is_hitler = Some(false);
//             p2.party_membership = Some(PartyMembership::Fascist);
//             p2.ready = false;
        
//             let mut p3 = Player::new(String::from("markus"), 2);
//             p3.connection_status = ConnectionStatus::Disconnected;
//             p3.is_hitler = Some(true);
//             p3.party_membership = Some(PartyMembership::Fascist);
//             p3.ready = false;
        
        
//             game_state.shared.players.push(p1);
//             game_state.shared.players.push(p2);
//             game_state.shared.players.push(p3);
        
//             game_state.shared.player_number = Some(3);
//             game_state.shared.fascist_known_by_hitler = Some(true);
//             game_state.shared.outboxes.insert(0, VecDeque::new());
//             game_state.shared.outboxes.insert(1, VecDeque::new());
//             game_state.shared.outboxes.insert(2, VecDeque::new());
//             game_state
//         }
//         _ => panic!("unknown seed state {}", state),
//     }

// }