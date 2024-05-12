use super::traits::PathTrait;

use std::{fs, io, path::PathBuf};

pub struct File {
    path: Box<PathBuf>,
}
impl File {}

impl PathTrait for File {
    fn get_path(&self) -> &Box<PathBuf> {
        &self.path
    }

    fn get_metadata(&self) -> Result<fs::Metadata, io::Error> {
        self.path.metadata()
    }

    fn get_name(&self) -> String {
        match self.path.file_name() {
            Some(name_str) => name_str.to_string_lossy().to_string(),
            None => self.path.to_string_lossy().to_string(),
        }
    }
}