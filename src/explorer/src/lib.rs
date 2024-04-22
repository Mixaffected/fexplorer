use std::path::{Path, PathBuf};
use std::{fs, io};

mod entry;
mod enums;

use entry::Entry;

pub struct Explorer {
    path: PathBuf,
    entries: Vec<Entry>,
}
impl Explorer {
    pub fn new(path: &Path) -> Result<Self, io::Error> {
        let entries = match Explorer::get_entries_from_path(&path) {
            Ok(entries) => entries,
            Err(e) => return Result::Err(e),
        };

        Ok(Self {
            path: path.to_owned(),
            entries,
        })
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_entries_from_path(path: &Path) -> Result<Vec<Entry>, io::Error> {
        let mut entries: Vec<Entry> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry = match Entry::new(&entry.path()) {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            entries.push(entry);
        }

        Ok(entries)
    }

    fn update_entries(&mut self) -> Result<(), io::Error> {
        self.entries = match Explorer::get_entries_from_path(&self.path) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn set_path(&mut self, path: &Path) -> Result<(), io::Error> {
        self.path = path.to_owned();

        match self.update_entries() {
            Ok(ok) => Ok(ok),
            Err(e) => Err(e),
        }
    }

    pub fn add_path(&mut self, path: &Path) -> Result<(), io::Error> {
        match self.set_path(&self.path.join(path)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn set_to_parent(&mut self) -> Result<(), io::Error> {
        let path = match self.path.parent() {
            Some(parent) => match parent.canonicalize() {
                Ok(parent) => parent,
                Err(e) => return Err(e),
            },
            None => self.path.to_owned(),
        };

        self.set_path(&path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_entries(explorer: &Explorer) {
        println!("[Location] {}", explorer.get_path().to_string_lossy());

        for entry in explorer.get_entries() {
            println!(
                "[{}] {}, {}, has_children: {}",
                entry.get_type(),
                entry.get_name().into_string().unwrap(),
                entry.get_path().to_string_lossy(),
                entry.has_children(),
            );
        }
    }

    #[test]
    fn new() {
        let explorer = Explorer::new(Path::new("/home")).unwrap();
        print_entries(&explorer);
    }

    #[test]
    fn test_parent() {
        let mut explorer = Explorer::new(Path::new("/home")).unwrap();

        explorer.set_to_parent().unwrap();
        print_entries(&explorer);
    }

    #[test]
    fn add_path() {
        let mut explorer = Explorer::new(Path::new("/home/xcf/Documents")).unwrap();
        let entry = explorer.get_entries().get(0).unwrap();

        let rel_path = entry.get_rel_path().unwrap();
        explorer.add_path(&rel_path).unwrap();
        print_entries(&explorer);
    }
}