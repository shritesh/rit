use chrono::{DateTime, Local};

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

    pub fn to_string(&self) -> String {
        format!(
            "{} <{}> {}",
            self.name,
            self.email,
            self.time.format("%s %z")
        )
    }
}
