use std::io::prelude::*;
use std::net::TcpStream;


use std::env;



mod testing;



// #[derive(Debug)]
// struct MessageAndDuration {
//     message: Message,
//     duration: f64,
// }


// fn read_user_actions(file_name: &str) -> std::io::Result<Vec<MessageAndDuration>> {

//     let file_content: String = fs::read_to_string(file_name)?.parse().unwrap();
//     let yaml = YamlLoader::load_from_str(&file_content).unwrap();
//     let root = yaml.first().unwrap();


//     let parsed_actions = root.clone().into_vec().unwrap();
//     let mut user_actions: Vec<MessageAndDuration> = Vec::new();

//     for transition in parsed_actions.iter() {
//         let mut state_name: Option<String> = None;
//         let mut user_name: Option<String> = None; 
//         let mut duration: Option<f64> = None; 

//         if let yaml_rust::yaml::Yaml::String(s) = transition["name"].clone() {
//             state_name = Some(s);
//         }
//         if let yaml_rust::yaml::Yaml::String(s) = transition["user_name"].clone() {
//             user_name = Some(s);
//         }
//         if let yaml_rust::yaml::Yaml::Real(f) = transition["duration"].clone() {
//             duration = Some(f.parse::<f64>().unwrap());
//         }


//         let state = state_name.unwrap();

//         let message: Message = match state.as_str() {
//             "connect" => Message::Connect{ user_name: user_name.unwrap().clone() },
//             "quit" => Message::Quit{ user_name: user_name.unwrap().clone() },
//             _ => panic!("unknown state: {} found. ", state),
//         };


//         user_actions.push(
//             MessageAndDuration {
//                 message: message,
//                 duration: duration.unwrap(),
//             }
//         );
//     }

//     Ok(user_actions)
// }


fn main() -> std::io::Result<()> {
    // let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    // let message = Message::Connect{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&message).unwrap();
    // let _result = stream.write(&mut serialized);
    
    // let message = Message::Quit{user_name: String::from("Lukas")};
    // let mut serialized = serde_json::to_vec(&message).unwrap();
    // let _result = stream.write(&mut serialized);

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("usage: cmd [filename.yaml]");
    }

    let user_actions = testing::read_user_actions(&args[1]).unwrap();

    for message_and_duration in user_actions.iter() {
        println!("{:?}", message_and_duration.message);
        println!("duration: {}s", message_and_duration.duration);
    }


    Ok(())
    //test
} // the stream is closed here

