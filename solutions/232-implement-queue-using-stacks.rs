#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    ReadyToPush,
    ReadyToPop,
}

struct MyQueue {
    stack: Vec<i32>,
    state: State,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            stack: Vec::new(),
            state: State::ReadyToPush,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.state == State::ReadyToPop {
            self.stack.reverse();
            self.state = State::ReadyToPush;
        }

        self.stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.state == State::ReadyToPush {
            self.stack.reverse();
            self.state = State::ReadyToPop;
        }

        self.stack.pop().unwrap()
    }
    
    fn peek(&self) -> i32 {
        match self.state {
            State::ReadyToPush => *self.stack.first().unwrap(),
            State::ReadyToPop => *self.stack.last().unwrap(),
        }
    }
    
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */