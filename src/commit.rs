use crate::{object::Object, Author, OId};

pub struct Commit {
    oid: OId,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(oid: OId, author: Author, message: impl Into<String>) -> Self {
        Self {
            oid,
            author,
            message: message.into(),
        }
    }
}

impl Object for Commit {
    fn kind(&self) -> &str {
        "commit"
    }

    fn data(&self) -> Vec<u8> {
        format!(
            "tree {}\nauthor {}\ncommitter {}\n\n{}",
            self.oid, self.author, self.author, self.message
        )
        .into_bytes()
    }
}
