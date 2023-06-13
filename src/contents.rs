use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct NotAvailableError;

impl std::error::Error for NotAvailableError {}
impl std::fmt::Display for NotAvailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File 'not available' error")
    }
}

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
            match state {
                FileState::None => {
                    if line.starts_with("```") {
                        state = FileState::CodeBlock;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    } else if line.starts_with("---") {
                        state = FileState::Comments;
                    } else if !line.starts_with('#') && !line.is_empty() {
                        state = FileState::Sentence;
                        sentence = String::new();
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
                FileState::CodeBlock => {
                    sentence.push_str(line);
                    if line.starts_with("```") {
                        contents.push(sentence);
                        sentence = String::new();
                        state = FileState::None;
                    }
                }
                FileState::Comments => {
                    if line.starts_with("---") {
                        state = FileState::None;
                    }
                }
                FileState::Sentence => {
                    if line.is_empty() {
                        state = FileState::None;
                        contents.push(sentence);
                        sentence = String::new();
                    } else {
                        sentence.push_str(line);
                        sentence.push('\n');
                    }
                }
            }
        }
        self.sentences = contents;
    }
}

// Load files from directory by ending
pub fn load_files_from_dir(dir: PathBuf, ending: &str) -> Result<Vec<File>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_files = load_files_from_dir(path, ending)?;
            files.append(&mut sub_files);
        } else {
            let path_clone = path.clone();
            let path_clone = Path::new(path_clone.as_os_str())
                .strip_prefix("/Users/stefan.baumgartner/Projects/Rust/qdrant-shuttle")?;
            let path_str = path_clone.to_str().ok_or(NotAvailableError {})?;
            if path.is_file() && path_str.ends_with(ending) {
                // Load file contents into string
                println!("Path: {:?}", path);
                let contents = fs::read_to_string(path)?;
                let mut file = File::new(path_str.to_string(), contents);
                file.parse();
                files.push(file);
            }
        }
    }
    Ok(files)
}
