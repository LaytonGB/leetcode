struct MyStack {
    s: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Self { s: Vec::with_capacity(100) }
    }
    
    fn push(&mut self, x: i32) {
        self.s.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        self.s.pop().unwrap()
    }
    
    fn top(&self) -> i32 {
        self.s[self.s.len()-1]
    }
    
    fn empty(&self) -> bool {
        self.s.len() == 0
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */