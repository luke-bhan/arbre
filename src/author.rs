use std::fmt;
use chrono::prelude::*;

pub struct Author{
    name: String,
    email: String,
    time: DateTime<Local>,
}

impl Author {
    pub fn new(name: String, email: String, time: DateTime<Local>) -> Author {
        Author{name, email, time}
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <{}> {}", self.name, self.email, self.time.format("%s %z").to_string())
    }
}

