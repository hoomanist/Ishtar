use super::*;

#[test]
fn test_push_instruct() {
    let stack = stack::new_stack();
    let returned = instructions::instruction_handler(stack, instructions::Instructions::PSH, vec![1]);
    let mut stack = returned.stack;
    let expected = vec![0, 1];
    assert_eq!(expected, stack.dump());
}
#[test]
fn test_pop_instruct() {
    let mut stack = stack::new_stack();
    stack.push(1);
    let returned = instructions::instruction_handler(stack, instructions::Instructions::POP, vec![]);
    let mut stack = returned.stack;
    assert_eq!(stack.dump(), vec![0]);
}
#[test]    
fn test_add_instruct() {       
    let stack = stack::new_stack();
    let returned = instructions::instruction_handler(stack, instructions::Instructions::ADD, vec![1, 3]);
    let mut stack = returned.stack;
    assert_eq!(stack.dump(), vec![0, 4]);
}
#[test]
fn test_sub_instruct() {
    let stack = stack::new_stack();
    let returned = instructions::instruction_handler(stack, instructions::Instructions::SUB, vec![16, 4]);
    let mut stack = returned.stack;
    assert_eq!(stack.dump(), vec![0, 12]);
}
