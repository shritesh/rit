use crate::{Author, OId, Object};

pub struct Commit {
    parent: Option<OId>,
    tree: OId,
    author: Author,
    message: String,
}

impl Commit {
    pub fn new(parent: Option<OId>, tree: OId, author: Author, message: impl Into<String>) -> Self {
        Self {
            parent,
            tree,
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
        let mut output = String::new();

        output.push_str(&format!("tree {}\n", self.tree));

        if let Some(parent) = &self.parent {
            output.push_str(&format!("parent {}\n", parent));
        }

        output.push_str(&format!(
            "author {}\ncommitter {}\n\n{}",
            self.author, self.author, self.message
        ));

        output.into_bytes()
    }
}
