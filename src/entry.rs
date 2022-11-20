use crate::OId;
pub struct Entry {
    pub name: String,
    pub oid: OId,
}

impl Entry {
    pub fn new(name: impl Into<String>, oid: OId) -> Self {
        Self {
            name: name.into(),
            oid,
        }
    }
}
