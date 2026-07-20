use std::cell::RefCell;
use std::rc::Rc;

struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

struct MyCircularQueue {
    back: Option<Rc<RefCell<ListNode>>>,
    front: Option<Rc<RefCell<ListNode>>>,
    len: i32,
    lim: i32,
}

#[inline(always)]
fn rc<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            back: None,
            front: None,
            len: 0,
            lim: k,
        }
    }
    
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        
        let mut node = Some(rc(ListNode {
            val: value,
            next: None,
        }));

        if let Some(back) = self.back.as_ref() {
            back.borrow_mut().next = node.clone();
        }
        self.back = node.clone();

        if self.front.is_none() {
            self.front = node;
        }

        self.len += 1;

        true
    }
    
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.front = self.front.take().unwrap().borrow_mut().next.take();
        self.len -= 1;

        if self.is_empty() {
            self.back = None;
        }

        return true;
    }
    
    fn front(&self) -> i32 {
        self.front.as_ref().map(|n| n.borrow().val).unwrap_or(-1)
    }
    
    fn rear(&self) -> i32 {
        self.back.as_ref().map(|n| n.borrow().val).unwrap_or(-1)
    }
    
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    fn is_full(&self) -> bool {
        self.len == self.lim
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