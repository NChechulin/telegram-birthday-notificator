use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sqlite_db_path: String,
    pub telegram_bot_token: String,
    pub telegram_chat_ids: Vec<i32>,
    pub greeting_templates: Vec<String>,
}

impl Config {
    pub fn load(path: &'static str) -> Config {
        let file = File::open(path).expect("No config file was found");
        serde_json::from_reader(file).expect("Error while parsing the config file")
    }
}
