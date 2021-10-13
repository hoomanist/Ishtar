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
    head: Pointer,
}

pub fn new_stack() -> Stack {
   let l = Vec::new();
   Stack{
    list: l,
    head: Pointer::Empty,
   }
}

impl Stack {
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
            let head = self.head.clone();
            node = Node{
                next: head,
                data: data,
            };
        }
        self.list.push(node.clone());
        self.head = Pointer::Pointy(Box::new(node));
    }
    pub fn pop(&mut self) -> Node {
        let current_haed = self.list[self.list.len() - 1].clone();
        self.list.pop();
        let new_head = self.list[self.list.len() - 1].clone();
        self.head = Pointer::Pointy(Box::new(new_head));
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
        let mut expected = Vec::new();
        expected.push(1);
        expected.push(15);
        assert_eq!(expected, stack.dump());
    }

    #[test]
    fn test_pop() {
        let mut stack = new_stack();
        stack.push(1);
        stack.push(15);
        stack.push(42);
        let latest = stack.pop();
        let mut expected = Vec::new();
        expected.push(1);
        expected.push(15);
        assert_eq!(expected, stack.dump());
        assert_eq!(latest.data, 42)
    }

}
