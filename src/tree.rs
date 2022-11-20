use crate::{Entry, Object};

static MODE: &str = "100644";

pub struct Tree {
    entries: Vec<Entry>,
}

impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        let mut entries = entries;
        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Self { entries }
    }
}

impl Object for Tree {
    fn kind(&self) -> &str {
        "tree"
    }

    fn data(&self) -> Vec<u8> {
        let mut content = Vec::new();

        for entry in &self.entries {
            content.extend(format!("{MODE} {}\0", entry.name).as_bytes());
            content.extend(entry.oid.as_bytes())
        }

        content
    }
}
