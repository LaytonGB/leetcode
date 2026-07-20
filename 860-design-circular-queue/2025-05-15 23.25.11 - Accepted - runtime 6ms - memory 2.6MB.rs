struct MyCircularQueue(Vec<i32>);


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        Self(Vec::with_capacity(k as usize))
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        
        self.0.push(value);

        true
    }
    
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.0.remove(0);

        true
    }
    
    fn front(&self) -> i32 {
        *self.0.first().unwrap_or(&-1)
    }
    
    fn rear(&self) -> i32 {
        *self.0.last().unwrap_or(&-1)
    }
    
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    
    fn is_full(&self) -> bool {
        self.0.len() == self.0.capacity()
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */