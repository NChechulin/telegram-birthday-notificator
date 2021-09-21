mod bot;
mod config;
mod user;

use crate::bot::TelegramBot;
use crate::user::User;
use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use tokio;

/// Converts a string like `"01-01-2021"` into a date object
fn string_to_date(date: String) -> NaiveDate {
    let tokens: Vec<&str> = date.split('-').collect();
    let year: i32 = tokens[0].parse().unwrap();
    let month: u32 = tokens[1].parse().unwrap();
    let day: u32 = tokens[2].parse().unwrap();
    NaiveDate::from_ymd(year, month, day)
}

/// Reads users from the database
fn read_user_list(path: String) -> Result<Vec<User>, &'static str> {
    let conn = match Connection::open(path) {
        Ok(val) => val,
        Err(_) => {
            return Err("Could not connect to database");
        }
    };

    let mut raw_data = match conn.prepare("SELECT * FROM Users") {
        Ok(val) => val,
        Err(_) => {
            return Err("Could not select users from the table");
        }
    };

    Ok(raw_data
        .query_map([], |row| {
            Ok(User {
                full_name: row.get(0).unwrap(),
                handle: row.get(1).unwrap(),
                birthday: string_to_date(row.get(2).unwrap()),
            })
        })
        .unwrap()
        .map(|usr| usr.unwrap())
        .collect::<Vec<User>>())
}

#[tokio::main]
async fn main() {
    let config = config::Config::load("config.json");
    let users = read_user_list(config.sqlite_db_path).unwrap();
    let bot = TelegramBot::new(&config.telegram_bot_token, config.telegram_chat_ids);

    for user in users {
        if user.is_birthday() {
            bot.wish_a_happy_bday(user).await;
        }
    }

    /*match bot.send_message_to_chat("hi bro".to_string(), CHAT_ID_WAS_HERE).await {
        Ok(_) => println!("OK"),
        Err(msg) => println!("Err: {}", msg),
    }
     */
}
