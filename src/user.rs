use chrono::prelude::Local as LocalTime;
use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub struct User {
    pub(crate) full_name: String,
    pub(crate) handle: String,
    pub(crate) birthday: NaiveDate,
}

impl User {
    pub fn new() -> User {
        User {
            full_name: String::from("Test User"),
            handle: String::from("@testusr"),
            birthday: NaiveDate::from_ymd(2005, 12, 31),
        }
    }

    pub fn is_birthday(&self) -> bool {
        let today = LocalTime::today();
        today.month() == self.birthday.month() && today.day() == self.birthday.day()
    }
}
