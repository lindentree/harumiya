use anyhow::Result;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::vector_db::errors::NotAvailableError;
use lopdf::Document;

pub struct File {
    pub path: String,
    pub contents: String,
    pub sentences: Vec<String>,
}

enum FileState {
    None,
    CodeBlock,
    Sentence,
    Comments,
}

impl File {
    pub fn new(path: String, contents: String) -> Self {
        Self {
            path,
            contents,
            sentences: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        let mut contents = Vec::new();
        let mut state = FileState::None;
        let mut sentence = String::new();

        for line in self.contents.lines() {
            //println!("LINE: {}", line);
            if line.is_empty() {
                state = FileState::None;
                contents.push(sentence);
                sentence = String::new();
            } else {
                sentence.push_str(line);
                sentence.push('\n');
            }
        }
        self.sentences = contents;
        //println!("Sentences inner: {:?}", self.sentences);
    }
}

trait HasFileExt {
    fn has_file_extension(&self, ending: &str) -> bool;
}

impl HasFileExt for Path {
    fn has_file_extension(&self, ending: &str) -> bool {
        if let Some(path) = self.to_str() {
            return path.ends_with(ending);
        }
        false
    }
}

pub fn load_txt_files_from_dir(dir: PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            println!("Path Directory: {:?}", path);
            let mut sub_files = load_txt_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path.is_file() && path.has_file_extension(ending) {
            println!("Path: {:?}", path);
            let contents = fs::read_to_string(&path)?;
            println!("Text: {}", contents);

            let path = Path::new(&path).strip_prefix(prefix)?.to_owned();
            let key = path.to_str().ok_or(NotAvailableError {})?;
            let mut file = File::new(key.to_string(), contents);
            file.parse();
            files.push(file);
        }
    }
    Ok(files)
}

pub fn load_pdf_files_from_dir(dir: PathBuf, ending: &str, prefix: &PathBuf) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            println!("Path Directory: {:?}", path);
            let mut sub_files = load_txt_files_from_dir(path, ending, prefix)?;
            files.append(&mut sub_files);
        } else if path.is_file() && path.has_file_extension(ending) {
            println!("Path: {:?}", path);
            let doc = Document::load(&path);
            let page_number: &[u32] = &[3];

            println!("PDF contents{}", doc.unwrap().extract_text(page_number)?);
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
    fn test_load_txt_files_from_dir() {
        let files =
            load_txt_files_from_dir(PathBuf::from("./documents"), ".txt", &PathBuf::from("."))
                .unwrap_or_else(|err| {
                    panic!("Failed to load files: {:?}", err);
                });
        //assert_eq!(files.len(), 1);
    }
}
