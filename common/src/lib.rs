use std::fs;
use yaml_rust::{YamlLoader};

use serde::{Serialize, Deserialize};
// use serde_json;

// pub mod another;
pub mod another;

pub fn say() -> another::Hello {
    another::Hello{x:2}
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ClientMessage {
    Hello,
    Connect {name: String},
    Chat {message: String},
    StillAlive,
    Ready {ready: bool},
    Quit,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PartyMembership {
    Fascist,
    Liberal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub connection_status: ConnectionStatus,
    pub thread_id: usize,
    pub ready: bool,
    pub party_membership: Option<PartyMembership>,
    pub is_hitler: Option<bool>,
}

impl Player {
    pub fn new(player_id: String, thread_id: usize) -> Player {
        Player {
            player_id: player_id,
            connection_status: ConnectionStatus::Connected,
            thread_id: thread_id,
            ready: false,
            party_membership: None,
            is_hitler: None,
        }
    }
}



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ServerMessage {
    Connected { user_name: String },
    Quit {user_name: String},
    Kicked {reason: String},             // player gets kicked from the server
    Reconnected {user_name: String},     // player reconnects to old session
    Rejected {reason: String},           //
    StatusUpdate {players: Vec<Player>}, // regular update of selected game state
    Advance,                             // server pushes users to next state
    Chat {user_name: String, message: String}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub server_address_and_port: String,
    pub server_listen_address_and_port: String,
    pub enable_debug_console: bool,
}

impl Configuration {
    pub fn create_from_configfile(file_name: &str) -> std::io::Result<Configuration>{
        // server address
        // server listen
        // server port

        // Configuration::parse_config(file_name);


        let file_content: String = fs::read_to_string(file_name)?.parse().unwrap();
        let yaml = YamlLoader::load_from_str(&file_content).unwrap();
        let root = yaml.first().unwrap();

        // server_address: 127.0.0.1
        // server_listen: 127.0.0.1
        // port: 54327

        // println!("config: {:?}", root);

        let mut server_address: Option<String> = None;
        let mut server_listen: Option<String> = None;
        let mut tmp_port: Option<String> = None;
        let mut enable_debug_console: Option<bool> = None;


        // TODO catch wrong config files, and print reasonable output message
        if let yaml_rust::yaml::Yaml::String(s) = root["server_address"].clone() {
            server_address = Some(s);
        }
        if let yaml_rust::yaml::Yaml::String(s) = root["server_listen"].clone() {
            server_listen = Some(s);
        }
        if let yaml_rust::yaml::Yaml::String(s) = root["port"].clone() {
            tmp_port = Some(s);
        }
        if let yaml_rust::yaml::Yaml::Boolean(b) = root["enable_debug_console"].clone() {
            enable_debug_console = Some(b);
        }

        let port = tmp_port.unwrap();

        Ok (Self {
            server_address_and_port: String::from(format!("{}:{}", server_address.unwrap(), port)),
            server_listen_address_and_port: String::from(format!("{}:{}", server_listen.unwrap(), port)),
            enable_debug_console: enable_debug_console.unwrap(),
        })
    }

}
