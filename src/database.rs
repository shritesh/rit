use crate::Object;
use deflate::deflate_bytes_zlib;
use rand::{seq::SliceRandom, thread_rng};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, OpenOptions},
    io::{Error, ErrorKind, Write},
    path::PathBuf,
};

pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn store(&self, object: impl Object) -> Result<(), Error> {
        let mut content =
            format!("{} {}\0", object.object_type(), object.data().len()).into_bytes();
        content.extend(object.data());

        let oid = sha1digest(&content);
        self.write_object(&oid, &content)
    }

    fn write_object(&self, oid: &str, content: &[u8]) -> Result<(), Error> {
        let oid_parts = oid.split_at(2);

        let parent = self.path.join(oid_parts.0);
        let object_path = parent.join(oid_parts.1);
        let temp_path = parent.join(generate_temp_name());

        let mut options = OpenOptions::new();
        options.read(true).write(true).create_new(true);

        let mut file = match options.open(&temp_path) {
            Err(e) if e.kind() == ErrorKind::NotFound => {
                fs::create_dir(parent)?;
                options.open(&temp_path)
            }
            f => f,
        }?;

        let compressed = deflate_bytes_zlib(content);
        file.write_all(&compressed)?;

        fs::rename(temp_path, object_path)?;

        Ok(())
    }
}

fn generate_temp_name() -> String {
    let temp_chars: Vec<char> = ('A'..='Z').chain('a'..='z').chain('0'..='9').collect();
    let rest: String = temp_chars.choose_multiple(&mut thread_rng(), 6).collect();
    format!("tmp_obj_{rest}")
}

fn sha1digest(input: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}
