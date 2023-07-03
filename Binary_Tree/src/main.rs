use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::Error;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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

struct Tree {
    root: Option<Rc<RefCell<Node>>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }
}

fn test() {
    let mut root = Tree::new();
}

fn main() {
    test();
}
