use std::{fs::File, io::Write};

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PlayerData{ 
    name: String , id:usize,
}

fn main () { 
    let p = PlayerData {
        name: String::from("K"),
        id: 8888,
    };

    let serialized = serde_json::to_string(&p).unwrap();

    let mut file = File::create("player_data.json").unwrap();

    file.write_all(&serialized.as_bytes()).unwrap();
}