use std::fs::*;
use std::io;
use std::error::Error;
use super::*;

pub struct Opcodes {
    instruct: instructions::Instructions,
    oprands: Vec<i32>
}

pub struct ParsedFile {
    data: Vec<Opcodes>,
}

pub struct Parser {
    pub contents: Option<String>,
}

impl Parser {
    pub fn open(&mut self, path: &str) {
        let opened_file = read_to_string(path).expect("file dosen't exist");
        self.contents = Some(opened_file);
    }
    pub fn parse(&mut self) -> Vec<Opcodes> {
        vec![Opcodes{
            instruct: instructions::Instructions::PSH,
            oprands: vec![1, 2],
        }]
    }
}

pub fn handle_file(path: String) -> ParsedFile{
    let mut parser = Parser{contents: None};
    let parsed_data = parser.parse();
    println!("{}", parser.contents.unwrap());
    ParsedFile {
        data: parsed_data,
    }
    
}
