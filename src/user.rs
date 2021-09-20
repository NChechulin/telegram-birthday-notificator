use chrono::NaiveDate;

#[derive(Debug)]
pub struct User {
    full_name: String,
    handle: String,
    birthday: NaiveDate,
}

impl User {
    pub fn new() -> User {
        User {
            full_name: String::from("Test User"),
            handle: String::from("@testusr"),
            birthday: NaiveDate::from_ymd(2005, 12, 31),
        }
    }
}
