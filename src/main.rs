mod stack;

fn main() {
    let mut stack = stack::new_stack();
    stack.dump();
    stack.push(12);
    stack.push(16);
    stack.push(18);
    stack.push(11);
    stack.dump();
    println!("****");
    stack.pop();
    stack.dump();

}
