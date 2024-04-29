use std::path::{Path, PathBuf};

use super::error::Error;
use crate::explorer::enums::EntryType;

pub trait BasicEntry {
    fn new(path: &Path) -> Result<Self, Error>
    where
        Self: Sized;

    fn get_type(&self) -> &EntryType;

    fn get_name(&self) -> Option<String>;

    fn get_path(&self) -> PathBuf;

    fn get_rel_path(&self) -> Result<PathBuf, Error>;

    fn has_children(&self) -> bool;

    fn get_entry_type_from_path(path: &Path) -> EntryType {
        if path.is_dir() {
            return EntryType::Directory;
        } else if path.is_file() {
            return EntryType::File;
        } else if path.is_symlink() {
            return EntryType::Link;
        } else {
            return EntryType::Unknown;
        };
    }
}