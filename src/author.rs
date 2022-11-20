use chrono::{DateTime, Local};
use std::fmt;

pub struct Author {
    pub name: String,
    pub email: String,
    pub time: DateTime<Local>,
}

impl Author {
    pub fn new(name: impl Into<String>, email: impl Into<String>, time: DateTime<Local>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            time,
        }
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} <{}> {}",
            self.name,
            self.email,
            self.time.format("%s %z")
        )
    }
}
