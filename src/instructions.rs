use crate::stack as stack_mod;

pub enum Instructions {
    PSH,
    POP,
    ADD,
    SUB,
}


pub struct Return {
    pub returned: bool,
    pub stack: stack_mod::Stack,
}

fn push_instruction(mut stack: stack_mod::Stack ,operands: Vec<i32>) -> Return {
    if operands.len() == 1 {
        stack.push(operands[0]);
        Return{
            returned: true,
            stack: stack,
        }
    } else {
        Return{
            returned: false,
            stack: stack
        }
    }
}

fn pop_instruction(mut stack: stack_mod::Stack, operands: Vec<i32>) -> Return {
    if operands.len() == 0 {
        stack.pop();
        Return {
            returned: true,
            stack: stack,
        }
    } else {
        Return{
            returned: false,
            stack: stack
        }
    }

}

fn add_instruction(mut stack: stack_mod::Stack, operands: Vec<i32>) -> Return {
    if operands.len() != 2 {
        Return{
            returned: false,
            stack: stack
        }
    } else {
        for operand in operands.iter() {
            stack.push(*operand);
        }
        let result = stack.pop().data + stack.pop().data;
        stack.push(result);
        Return {
            returned: true,
            stack: stack,
        }
    }
}

fn sub_instruction(stack: stack_mod::Stack, operands: Vec<i32>) -> Return {
    if operands.len() != 2 {
        Return{
            returned: false,
            stack: stack
        }
    } else {
        add_instruction(stack, vec![operands[0], -operands[1]])
    }
}


pub fn instruction_handler(stack: stack_mod::Stack, instruct: Instructions, operands: Vec<i32>) -> Return {
    match instruct {
        Instructions::PSH => push_instruction(stack, operands),
        Instructions::POP => pop_instruction(stack, operands),
        Instructions::ADD => add_instruction(stack, operands),
        Instructions::SUB => sub_instruction(stack, operands),
    }
}


