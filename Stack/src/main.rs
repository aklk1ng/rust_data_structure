struct MyStack {
    queue: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack { queue: vec![] }
    }

    fn push(&mut self, val: i32) {
        self.queue.push(val)
    }

    fn pop(&mut self) -> i32 {
        let len = self.queue.len() - 1;
        for _ in 0..len {
            let tmp = self.queue.remove(0);
            self.queue.push(tmp);
        }
        self.queue.remove(0)
    }

    fn top(&mut self) -> i32 {
        let res = self.queue.pop().unwrap();
        self.queue.push(res);
        res
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn print(&self) {
        for i in self.queue.iter() {
            println!("{}", i);
        }
        println!("---------------");
    }
}

fn test() {
    let mut st = MyStack::new();
    st.push(0);
    st.push(1);
    st.push(2);
    st.push(3);
    st.pop();
    st.print();
    println!("{}", st.top());
    assert!(!st.empty());
}

fn main() {
    test();
}
