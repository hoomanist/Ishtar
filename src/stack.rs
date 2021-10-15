#[derive(Clone)]
pub struct Node {
    next: Pointer,
    data: i32,
}
#[derive(Clone)]
enum Pointer{
    Empty,
    Pointy(Box<Node>),
}
#[derive(Clone)]
pub struct Stack {
    list: Vec<Node>,
    stack_pointer: Pointer,
}

pub fn new_stack() -> Stack {
   let l = Vec::new();
   Stack{
    list: l,
    stack_pointer: Pointer::Empty,
   }
}

impl Stack {
    pub fn is_empty(&mut self) -> bool {
        self.list.is_empty()
    }
    pub fn dump(&mut self) -> Vec<i32> {
        let mut dumped = Vec::new();
        for node in self.list.iter() {
            dumped.push(node.data);
        }
        dumped

    }
    pub fn push(&mut self, data: i32) {
        let node: Node;
        // check whether it is first node or not
        if self.list.len() == 0 {
            node = Node{
                next: Pointer::Empty,
                data: data,
            };
        } else {
            let stack_pointer= self.stack_pointer.clone();
            node = Node{
                next: stack_pointer,
                data: data,
            };
        }
        self.list.push(node.clone());
        self.stack_pointer = Pointer::Pointy(Box::new(node));
    }
    pub fn pop(&mut self) -> Node {
        let current_haed = self.list[self.list.len() - 1].clone();
        self.list.pop();
        let new_head = self.list[self.list.len() - 1].clone();
        self.stack_pointer = Pointer::Pointy(Box::new(new_head));
        current_haed
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = new_stack();
        stack.push(1);
        stack.push(15);
        let expected = vec![1, 15];
        assert_eq!(expected, stack.dump());
    }

    #[test]
    fn test_pop() {
        let mut stack = new_stack();
        stack.push(1);
        stack.push(15);
        stack.push(42);
        stack.push(121);
        assert_eq!(stack.pop().data, 121);
        assert_eq!(stack.pop().data, 42);
        let expected = vec![1, 15];
        assert_eq!(expected, stack.dump());
    }

    #[test]
    fn test_is_empty() {
        let mut stack = new_stack();
        assert_eq!(stack.is_empty(), true);
        stack.push(42);
        assert_eq!(stack.is_empty(), false);
    }
}

