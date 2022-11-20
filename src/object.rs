use sha1::{Digest, Sha1};
use std::{
    fmt::{self},
    io::{Error, ErrorKind},
};

#[derive(Clone, Copy)]
pub struct OId([u8; 20]);

impl OId {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(data);
        let result = hasher.finalize();
        Self(result.into())
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

impl TryFrom<String> for OId {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(v) = hex::decode(value) {
            if let Ok(bytes) = v.try_into() {
                return Ok(Self(bytes));
            }
        }

        Err(Error::from(ErrorKind::InvalidData))
    }
}

impl fmt::Display for OId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

pub trait Object {
    fn kind(&self) -> &str;

    fn data(&self) -> Vec<u8>;
}
