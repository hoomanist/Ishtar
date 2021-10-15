use crate::stack as stack_mod;

pub enum Instructions {
    PSH,
    POP,
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

pub fn instruction_handler(stack: stack_mod::Stack, instruct: Instructions, operands: Vec<i32>) -> Return {
    match instruct {
        Instructions::PSH => push_instruction(stack, operands),
        Instructions::POP => pop_instruction(stack, operands),
    }
}


