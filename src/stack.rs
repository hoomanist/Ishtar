#[derive(Clone)]
struct Node {
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
    pub fn dump(&mut self) {
        for node in self.list.iter() {
            println!("{}", node.data)
        }
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
    pub fn pop(&mut self) {
        self.list.pop();
    }
    
}
