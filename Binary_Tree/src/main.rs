use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

fn test() {
    let mut root = Node::new(0);
}

fn main() {
    test();
}
