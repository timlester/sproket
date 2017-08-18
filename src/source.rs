use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

pub struct SourceManager {
    pub source_files: Vec<SourceFile>
}

impl SourceManager {
    pub fn new() -> SourceManager {
        SourceManager { source_files: vec![] }
    }

    pub fn add_source_file(&mut self, file: &str) {
        let path = Path::new(file);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(e) => panic!("Error: Unable to open {}: {}", display, e.description()),
            Ok(file) => file
        };

        let mut source_string = String::new();
        match file.read_to_string(&mut source_string) {
            Err(e) => panic!("Error: Unable to read {}: {}", display, e.description()),
            Ok(_) => { }
        }

        let source_file = SourceFile::new(display.to_string(), source_string);
        self.source_files.push(source_file);
    }
}

pub struct SourceFile {
    path: String,
    source: Vec<char>,
    line: usize,
    col: usize,
    index: usize
}

impl SourceFile {
    pub fn new(path: String, source_string: String) -> SourceFile {
        let chars: Vec<_> = source_string.chars().collect();
        SourceFile { path: path, source: chars, line: 1, col: 1, index: 0 }
    }

    pub fn eof(&self) -> bool {
        self.source.len() <= self.index
    }

    pub fn peek(&self) -> char {
        self.source[self.index]
    }

    pub fn get_next(&mut self) -> char {
        self.index += 1;
        let c = self.source[self.index - 1];
        
        match c {
            '\n' => { 
                self.line += 1; 
                self.col = 0;    
            },
            _ => { self.col += 1 }
        }

        c
    }

    pub fn throw_error(&self, error: &str, message: &str) {
        println!("Error: {}", error);

        if message.trim() != "" {
            println!("{}", message);
        }

        println!("{}:{}:{}", self.path, self.line, self.col);
        process::exit(1);
    }
}
