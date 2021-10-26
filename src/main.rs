mod stack ;
mod instructions;
mod file_analyzer;
mod repl;

fn main() {
    let main_stack = stack::new_stack();
    repl::repl(&main_stack);
}

#[cfg(test)]
mod tests ;
