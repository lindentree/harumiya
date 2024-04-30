use anyhow::Result;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::vector_db::errors::NotAvailableError;

// src/contents.rs
pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>,
}

impl File {
    pub fn new(path: String, contents: String) -> Self {
        File {
            path,
            contents,
            sentences: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        // Add your implementation here
    }
}

pub fn load_files_from_dir(dir: PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path.is_file() && path.extension().map(|ext| ext == ending).unwrap_or(false) {
            println!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            let key = path.to_str().ok_or(NotAvailableError {})?;
            let mut file = File::new(key.to_string(), contents);
            file.parse();
            files.push(file);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_file_new() {
        let path = String::from("../../documents/blueprint.pdf");
        let contents = String::from("This is the file contents.");
        let file = File::new(path.clone(), contents.clone());

        assert_eq!(file.path, path);
        assert_eq!(file.contents, contents);
        assert_eq!(file.sentences, Vec::<String>::new());
    }

    #[test]
    fn test_file_parse() {
        let mut file = File::new(
            String::from("/path/to/file.txt"),
            String::from("This is the file contents."),
        );
        file.parse();

        // Add assertions for the expected behavior of the `parse` method
    }
}
