mod stack ;
mod instructions;
mod file_analyzer;
mod repl;

use argh::FromArgs;

#[derive(FromArgs)]
/// A simple language vm
struct Args{
    /// repl mode or not
    #[argh(switch, short = 'r')]
    repl: bool,
    /// file
    #[argh(option)]
    file: Option<String>,
}

fn main() {
    let main_stack = stack::new_stack();
    let arguments: Args = argh::from_env();
    if arguments.repl && arguments.file.is_none() {
        repl::repl(&main_stack);
    }
}

#[cfg(test)]
mod tests ;
