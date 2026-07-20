#[derive(Debug, PartialEq, Eq)]
enum State {
    Push,
    Pop,
}

impl State {
    fn get_other(&self) -> Self {
        match self {
            Self::Push => Self::Pop,
            Self::Pop => Self::Push,
        }
    }
}

#[derive(Debug)]
struct MyQueue {
    dat: Vec<i32>,
    state: State,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            dat: vec![],
            state: State::Push,
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.state == State::Pop {
            self.toggle_state();
        }
        self.dat.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.state == State::Push {
            self.toggle_state();
        }
        self.dat.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.state == State::Push {
            self.toggle_state();
        }
        *self.dat.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.dat.is_empty() && self.dat.is_empty()
    }

    fn toggle_state(&mut self) {
        self.dat = self.dat.iter().copied().rev().collect();
        self.state = self.state.get_other();
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