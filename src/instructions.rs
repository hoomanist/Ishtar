use crate::stack as stack_mod;

#[derive(Clone, Debug, PartialEq)]
pub enum Instructions {
    PSH,
    POP,
    ADD,
    SUB,
}

fn push_instruction(mut stack: stack_mod::Stack ,operands: Vec<i32>) -> (bool, stack_mod::Stack){
    if operands.len() == 1 {
        stack.push(operands[0]);
        (true, stack)
    } else {
        (false, stack)
    }
}

fn pop_instruction(mut stack: stack_mod::Stack, operands: Vec<i32>) -> (bool, stack_mod::Stack) {
    if operands.len() == 0 {
        stack.pop();
        (true, stack)
    } else {
        (false, stack)
    }

}

fn add_instruction(mut stack: stack_mod::Stack, operands: Vec<i32>) -> (bool, stack_mod::Stack){
    if operands.len() != 2 {
        (true, stack)
    } else {
        for operand in operands.iter() {
            stack.push(*operand);
        }
        let result = stack.pop().data + stack.pop().data;
        stack.push(result);
        (false, stack)
    }
}

fn sub_instruction(stack: stack_mod::Stack, operands: Vec<i32>) -> (bool, stack_mod::Stack){
    if operands.len() != 2 {
        (false, stack)
    } else {
        add_instruction(stack, vec![operands[0], -operands[1]])
    }
}


pub fn instruction_handler(stack: stack_mod::Stack, instruct: Instructions, operands: Vec<i32>) -> (bool, stack_mod::Stack){
    match instruct {
        Instructions::PSH => push_instruction(stack, operands),
        Instructions::POP => pop_instruction(stack, operands),
        Instructions::ADD => add_instruction(stack, operands),
        Instructions::SUB => sub_instruction(stack, operands),
    }
}


