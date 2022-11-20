use sha1::{Digest, Sha1};

pub struct OId {
    bytes: [u8; 20],
    string: String,
}

impl OId {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        Self {
            bytes: result.into(),
            string: format!("{:x}", result),
        }
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.bytes
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }
}

pub trait Object {
    fn kind(&self) -> &str;

    fn data(&self) -> Vec<u8>;
}
