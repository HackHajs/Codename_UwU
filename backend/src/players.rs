use std::{
    fs::{ 
        read_to_string,
        write,
        File,
    },
    io::{
        ErrorKind,
        prelude::*,
    },
    collections::HashMap,
};

use regex::Regex;
use serde::{Deserialize, Serialize};

pub enum Field {
    Xpos(u32),
    Ypos(u32),
    Health(u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub x_pos: u32,
    pub y_pos: u32,
    pub health: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub players: HashMap<String, Player>,
}

pub fn parse_players() -> HashMap<String, Player> {
    // Get player save file path
    let mut player_file_path = home::home_dir().unwrap();
    player_file_path.push("Demo");
    player_file_path.push("players");

    // Read or create the save file
    let unparsed_players = match read_to_string(&player_file_path) {
        Ok(file) => file, 
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                File::create(&player_file_path).expect("Failed to create config file");
                String::new()
            },
            _ => panic!("Failed to create save file!"),
        }
    };

    let mut players: HashMap<String, Player> = HashMap::new();
    
    let re = Regex::new(r"\(([^:]+):(\d+):(\d+):(\d+)\)").unwrap();
    for cap in re.captures_iter(&unparsed_players) {
        let player = Player {
            name: cap[1].to_string(),
            health: cap[2].parse().unwrap(),
            x_pos: cap[3].parse().unwrap(),
            y_pos: cap[4].parse().unwrap(),
        };
        players.insert(cap[1].to_string(), player);
    }

    players
}

pub fn save_players(players: HashMap<String, Player>) { 
    let mut player_file_path = home::home_dir().unwrap();
    player_file_path.push("Demo");
    player_file_path.push("players");

    let mut save_file = String::new();
    for player in players.values() {
        let fmt_player = format!("({}:{}:{}:{})\n", player.name, player.health, player.x_pos, player.y_pos);
        save_file.push_str(&fmt_player);
    }
    let mut file = std::fs::File::create(player_file_path).unwrap();
    file.write_all(save_file.as_bytes()).unwrap();
}
