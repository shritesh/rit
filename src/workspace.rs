use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

pub struct Workspace {
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn list_files(&self) -> Result<Vec<String>, Error> {
        let mut result = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let fnamestr = entry?.file_name();

            let file_name = fnamestr
                .to_str()
                .ok_or_else(|| Error::new(ErrorKind::Other, "cannot convert path to string"))?;

            if file_name != ".git" {
                result.push(file_name.into());
            }
        }

        Ok(result)
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        fs::read(self.path.join(path))
    }
}
