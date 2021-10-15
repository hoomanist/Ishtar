mod stack ;
mod instructions;

fn main() {
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = stack::new_stack();
        stack.push(1);
        stack.push(15);
        let expected = vec![0, 1, 15];
        assert_eq!(expected, stack.dump());
    }

    #[test]
    fn test_pop() {
        let mut stack = stack::new_stack();
        stack.push(1);
        stack.push(15);
        stack.push(42);
        stack.push(121);
        assert_eq!(stack.pop().data, 121);
        assert_eq!(stack.pop().data, 42);
        let expected = vec![0, 1, 15];
        assert_eq!(expected, stack.dump());
    }

    #[test]
    fn test_is_empty() {
        let mut stack = stack::new_stack();
        assert_eq!(stack.is_empty(), true);
        stack.push(42);
        assert_eq!(stack.is_empty(), false);
    }

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
}

