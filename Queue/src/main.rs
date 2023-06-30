struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack_in.push(val)
    }

    fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap())
            }
        }
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        let res = self.pop();
        self.stack_out.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }

    fn print(&self) {
        for i in self.stack_in.iter() {
            println!("{}", i);
        }
        for i in self.stack_out.iter().rev() {
            println!("{}", i);
        }
        println!("-------------");
    }
}

fn test() {
    let mut q = MyQueue::new();
    q.push(0);
    q.push(1);
    q.push(2);
    q.push(3);
    q.print();
    q.pop();
    q.print();
    println!("{}", q.peek());
    assert!(!q.empty());
}

fn main() {
    test();
}
