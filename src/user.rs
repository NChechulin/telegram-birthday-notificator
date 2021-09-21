use chrono::prelude::Local as LocalTime;
use chrono::{Datelike, NaiveDate};

#[derive(Debug)]
pub struct User {
    pub(crate) full_name: String,
    pub(crate) handle: String,
    pub(crate) birthday: NaiveDate,
}

impl User {
    pub fn is_birthday(&self) -> bool {
        let today = LocalTime::today();
        today.month() == self.birthday.month() && today.day() == self.birthday.day()
    }
}
