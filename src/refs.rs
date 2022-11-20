use crate::OId;
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Write},
    path::PathBuf,
};

pub struct Refs {
    path: PathBuf,
}

impl Refs {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn update_head(&self, oid: &OId) -> Result<(), Error> {
        let mut file = File::create(self.head_path())?;
        file.write_all(oid.to_string().as_bytes())
    }

    pub fn read_head(&self) -> Result<Option<OId>, Error> {
        match fs::read_to_string(self.head_path()) {
            Ok(s) => Ok(Some(s.try_into()?)),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn head_path(&self) -> PathBuf {
        self.path.join("HEAD")
    }
}
