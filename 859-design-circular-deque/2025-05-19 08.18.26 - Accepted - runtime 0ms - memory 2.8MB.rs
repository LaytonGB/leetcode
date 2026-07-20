use std::cell::RefCell;
use std::rc::Rc;

struct ListNode {
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
    val: i32,
}

struct MyCircularDeque {
    front: Option<Rc<RefCell<ListNode>>>,
    back: Option<Rc<RefCell<ListNode>>>,
    len: usize,
    capacity: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            capacity: k as usize,
        }
    }
    
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        
        let mut front = self.front.take();
        match front {
            None => {
                self.front = Some(Rc::new(RefCell::new(
                    ListNode {
                        prev: None,
                        next: None,
                        val: value,
                    }
                )));
                self.back = self.front.clone();
            }
            Some(_) => {
                self.front = Some(Rc::new(RefCell::new(
                    ListNode {
                        prev: front,
                        next: None,
                        val: value,
                    }
                )));

                let prev = &self.front.as_ref().unwrap().borrow().prev;
                prev.as_ref().unwrap().borrow_mut().next = self.front.clone();
            }
        }

        self.len += 1;
        true
    }
    
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        
        let mut back = self.back.take();
        match back {
            None => {
                self.back = Some(Rc::new(RefCell::new(
                    ListNode {
                        prev: None,
                        next: None,
                        val: value,
                    }
                )));
                self.front = self.back.clone();
            }
            Some(_) => {
                self.back = Some(Rc::new(RefCell::new(
                    ListNode {
                        prev: None,
                        next: back,
                        val: value,
                    }
                )));
                
                let next = &self.back.as_ref().unwrap().borrow().next;
                next.as_ref().unwrap().borrow_mut().prev = self.back.clone();
            }
        }

        self.len += 1;
        true
    }
    
    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.front = self.front.take().unwrap().borrow_mut().prev.take();

        if let Some(ref front) = self.front {
            front.borrow_mut().next = None;
        } else {
            self.back = None;
        }
        
        self.len -= 1;
        true
    }
    
    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.back = self.back.take().unwrap().borrow_mut().next.take();

        if let Some(ref back) = self.back {
            back.borrow_mut().prev = None;
        } else {
            self.front = None;
        }

        self.len -= 1;

        if self.back.is_none() {
            self.front = None;
        }

        true
    }
    
    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        
        self.front.as_ref().unwrap().borrow().val
    }
    
    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        
        self.back.as_ref().unwrap().borrow().val
    }
    
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */