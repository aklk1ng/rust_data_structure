// singly linked list
#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        // haven't no dummny node
        Node { val, next: None }
    }

    fn delete_at_index(&mut self, index: i32) -> Option<T> {
        let mut i = 0;
        let mut cur = self;
        while let Some(node) = cur.next.take() {
            if i == index {
                cur.next = node.next;
                return Some(node.val);
            }
            i += 1;
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
        None
    }
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push_front(&mut self, val: T) {
        let mut newnode = Node::new(val);
        newnode.next = self.head.take();
        self.head = Some(Box::new(newnode));
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

fn test_slist_node() {
    let mut list = Node::new(0);
    list.delete_at_index(4);
}

fn main() {
    test_slist_node();
}

#[cfg(test)]
mod tests {
    use super::Node;
    #[test]
    fn test1() {
        let mut list = Node::new(-1);
    }
}
