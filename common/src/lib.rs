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
    StillAlive,
    Ready {ready: bool},
    Quit,
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
}

impl Player {
    pub fn new(player_id: String, thread_id: usize) -> Player {
        Player {
            player_id: player_id,
            connection_status: ConnectionStatus::Connected,
            thread_id: thread_id,
            ready: false,
        }
    }
}



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ServerMessage {
    Connected { user_name: String },
    Quit {user_name: String},
    Users {users: Vec<String>},
    Kicked {reason: String},
    Reconnected {user_name: String},
    Rejected {reason: String},
    StatusUpdate {players: Vec<Player>},
}

pub struct Configuration {
    pub server_address_and_port: String,
    pub server_listen_address_and_port: String,
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


        if let yaml_rust::yaml::Yaml::String(s) = root["server_address"].clone() {
            server_address = Some(s);
        }
        if let yaml_rust::yaml::Yaml::String(s) = root["server_listen"].clone() {
            server_listen = Some(s);
        }
        if let yaml_rust::yaml::Yaml::String(s) = root["port"].clone() {
            tmp_port = Some(s);
        }

        let port = tmp_port.unwrap();

        Ok (Self {
            server_address_and_port: String::from(format!("{}:{}", server_address.unwrap(), port)),
            server_listen_address_and_port: String::from(format!("{}:{}", server_listen.unwrap(), port)),
        })
    }

}