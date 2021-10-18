use std::fs::*;
use std::io;
use std::path::Path;
use super::*;

pub struct Opcodes {
    pub instruct: instructions::Instructions,
    pub oprands: Vec<i32>
}

pub struct ParsedFile {
    data: Vec<Opcodes>,
}

pub struct Parser {
    pub contents: Option<String>,
}

impl Parser {
    pub fn open(&mut self, path: &str) {
        let absoulte_path = Path::new(path);
        let opened_file = read_to_string(absoulte_path
                                         .canonicalize()
                                         .unwrap())
            .expect("file dosen't exist");
        self.contents = Some(opened_file);
    }
    pub fn parse(&mut self) -> Vec<Opcodes> {
        let mut parsed_command = vec![];
        for line in self.contents
            .clone()
            .unwrap()
            .to_string()
            .lines() {
                let splited = line.split_whitespace();
                let mut index = 0;
                let mut operands = vec![];
                let mut instruct = "";
                for word in splited{
                    if index > 0 {
                        operands.push(word.parse::<i32>().unwrap());
                        index += 1;
                    } else if index == 0 {
                        instruct = word;
                    }
                }
                let instruct_formatted = match instruct {
                    "PSH" => instructions::Instructions::PSH,
                    "POP" => instructions::Instructions::POP,
                    "ADD" => instructions::Instructions::ADD,
                    "SUB" => instructions::Instructions::SUB,
                    _ => std::process::exit(1),
                };
                parsed_command.push(Opcodes{
                    instruct: instruct_formatted,
                    oprands: operands,
                })
        }
        parsed_command
        
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
