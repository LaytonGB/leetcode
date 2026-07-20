struct ListNode {
    next: Option<Box<ListNode>>,
    val: i32,
}

struct MyLinkedList {
    head: Option<Box<ListNode>>,
    len: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }
    
    fn get(&self, index: i32) -> i32 {
        if index >= self.len {
            return -1;
        }

        let mut node = self.head.as_ref().unwrap();
        for _ in 0..index {
            node = node.next.as_ref().unwrap();
        }

        node.val
    }
    
    fn add_at_head(&mut self, val: i32) {
        self.len += 1;

        self.head = Some(Box::new(
            ListNode {
                next: self.head.take(),
                val,
            }
        ));
    }
    
    fn add_at_tail(&mut self, val: i32) {
        self.len += 1;

        if self.len == 1 {
            self.head = Some(Box::new(
                ListNode {
                    next: None,
                    val,
                }
            ));
            return;
        }

        let mut node = self.head.as_mut().unwrap();
        for _ in 0..self.len - 2 {
            node = node.next.as_mut().unwrap();
        }

        node.next = Some(Box::new(
            ListNode {
                next: None,
                val,
            }
        ));
    }
    
    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.len {
            return;
        }

        self.len += 1;

        if index == 0 {
            self.head = Some(Box::new(
                ListNode {
                    next: self.head.take(),
                    val,
                }
            ));
            return;
        }

        let mut node = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            node = node.next.as_mut().unwrap();
        }

        let next = node.next.take();
        node.next = Some(Box::new(
            ListNode {
                next,
                val,
            }
        ));

    }
    
    fn delete_at_index(&mut self, index: i32) {
        if index >= self.len {
            return;
        }

        self.len -= 1;

        if index == 0 {
            self.head = self.head.take().unwrap().next;
            return;
        }

        let mut node = self.head.as_mut().unwrap();
        for _ in 0..index - 1 {
            node = node.next.as_mut().unwrap();
        }

        node.next = node.next.take().and_then(|n| n.next);
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */