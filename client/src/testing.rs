use std::fs;
use yaml_rust::YamlLoader;

use common;

#[derive(Debug)]
pub struct MessageAndDuration {
    pub message: common::ClientMessage,
    pub duration: f64,
}

pub fn read_user_actions(file_name: &str) -> std::io::Result<Vec<MessageAndDuration>> {
    let file_content: String = fs::read_to_string(file_name)?.parse().unwrap();
    let yaml = YamlLoader::load_from_str(&file_content).unwrap();
    let root = yaml.first().unwrap();

    let parsed_actions = root.clone().into_vec().unwrap();
    let mut user_actions: Vec<MessageAndDuration> = Vec::new();

    for transition in parsed_actions.iter() {
        let mut state_name: Option<String> = None;
        let mut user_name: Option<String> = None;
        let mut duration: Option<f64> = None;

        if let yaml_rust::yaml::Yaml::String(s) = transition["name"].clone() {
            state_name = Some(s);
        }
        if let yaml_rust::yaml::Yaml::String(s) = transition["user_name"].clone() {
            user_name = Some(s);
        }
        if let yaml_rust::yaml::Yaml::Real(f) = transition["duration"].clone() {
            duration = Some(f.parse::<f64>().unwrap());
        }

        let state = state_name.unwrap();

        let message: common::ClientMessage = match state.as_str() {
            "connect" => common::ClientMessage::Connect {
                user_name: user_name.unwrap().clone(),
            },
            "quit" => common::ClientMessage::Quit {
                user_name: user_name.unwrap().clone(),
            },
            _ => panic!("unknown state: {} found. ", state),
        };

        user_actions.push(MessageAndDuration {
            message: message,
            duration: duration.unwrap(),
        });
    }

    Ok(user_actions)
}
// }
// }
