mod user;
use crate::user::User;
use chrono::NaiveDate;
use rusqlite::{Connection, Result};

/// Converts a string like `"01-01-2021"` into a date object
fn string_to_date(date: String) -> NaiveDate {
    let tokens: Vec<&str> = date.split('-').collect();
    let year: i32 = tokens[0].parse().unwrap();
    let month: u32 = tokens[1].parse().unwrap();
    let day: u32 = tokens[2].parse().unwrap();
    NaiveDate::from_ymd(year, month, day)
}

/// Reads users from the database
fn read_user_list() -> Result<Vec<User>, &'static str> {
    let conn = match Connection::open("/home/nikolay/Documents/dev/dsba201_bday_bot/db.db") {
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

fn main() {
    for user in read_people_list().unwrap() {
        println!("{:?}", user);
    }
}
