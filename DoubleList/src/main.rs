use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    val: T,
    pre: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            pre: None,
            next: None,
        }))
    }

    fn insert_after(&mut self, other: &mut Node<T>) {
        self.next = Some(Rc::clone(other));
        other.pre = Some(Rc::clone(self));
    }
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn push_front(&mut self, val: T) {
        let newnode = Rc::new(RefCell::new(Node::new(val)));
        if let Some(head) = self.head.take() {
            head.borrow_mut().pre = Some(Rc::clone(&newnode));
            newnode.borrow_mut().next = Some(head);
            self.head = Some(newnode);
            self.size += 1;
        } else {
            self.tail = Some(Rc::clone(&newnode));
            self.head = Some(newnode);
            self.size = 1;
        }
    }

    fn push_back(&mut self, val: T) {
        let newnode = Rc::new(RefCell::new(Node::new(val)));
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&newnode));
            newnode.borrow_mut().pre = Some(tail);
            self.tail = Some(newnode);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&newnode));
            self.tail = Some(newnode);
            self.size = 1;
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            self.size -= 1;
            match tail.borrow_mut().pre.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().val
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.size -= 1;
            match head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().pre = None;
                    self.tail = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().val
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().pre.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

fn test() {
    let mut list: List<i32> = List::new();
    for i in 0..=5 {
        list.push_back(i);
    }
}

fn main() {
    test();
}
