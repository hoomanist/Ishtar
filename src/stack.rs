#[derive(Clone)]
pub struct Node {
    pub data: i32,
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
            println!("{}", node.data);
        }
        dumped

    }
    pub fn push(&mut self, data: i32) {
        let node: Node;
        // check whether it is first node or not
        if self.list.len() == 0 {
            let initial_node = Node{
                data: 0,
            };
            self.list.push(initial_node.clone());
            node = Node{
                data: data,
            };
        } else {
            let stack_pointer= self.stack_pointer.clone();
            node = Node{
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
        println!("{}", current_haed.data);
        current_haed
    }
    
}

