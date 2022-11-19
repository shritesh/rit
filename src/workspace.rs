use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub struct Workspace {
    path: PathBuf,
}

impl Workspace {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn list_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut result = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let file_name = entry?.file_name();

            if file_name != ".git" {
                result.push(file_name.into());
            }
        }

        Ok(result)
    }

    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>, Error> {
        fs::read(self.path.join(path))
    }
}
