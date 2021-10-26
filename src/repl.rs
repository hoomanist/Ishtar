use std::io;
use linefeed::{Interface, ReadResult};
use super::*;

pub fn execute(data: Vec<file_analyzer::Opcodes>, current_stack: &stack::Stack) {
    if data.len() != 0 {
        let current_instruction = data[0].clone();
        instructions::instruction_handler(current_stack.clone(), current_instruction.instruct, current_instruction.oprands); 
        
    }else {
        std::process::exit(0);
    }

}

pub fn repl(current_stack: &stack::Stack) -> io::Result<()> {
    let reader = Interface::new("ishtar-repl")?;

    reader.set_prompt("> ")?;

    while let ReadResult::Input(input) = reader.read_line()? {
        let mut parser = file_analyzer::Parser{
            contents: Some(input),
        };
        let parsed_command = parser.parse();
        execute(parsed_command, current_stack);
    }

    Ok(())
}
