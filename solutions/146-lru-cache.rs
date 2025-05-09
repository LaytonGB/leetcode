use std::collections::*;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::mem;

#[derive(Debug)]
struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}
impl ListNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key,
            val: value,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    capacity: usize,
    length: usize,
}

fn new_rc_node(key: i32, value: i32) -> Rc<RefCell<ListNode>> {
    Rc::new(RefCell::new(ListNode::new(key, value)))
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;

        let head = new_rc_node(-1, -1);
        let tail = new_rc_node(-1, -1);
        {
            let mut head_mut = head.borrow_mut();
            head_mut.next = Some(tail.clone());
            let mut tail_mut = tail.borrow_mut();
            tail_mut.prev = Some(head.clone());
        }

        Self {
            head,
            tail,
            map: HashMap::with_capacity(capacity + 1),
            capacity,
            length: 0,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let mut res = -1;
        if let Some(node) = self.map.get(&key) {
            let node_clone = node.clone();
            mem::drop(node);
            
            let node_inner = node_clone.borrow();
            res = node_inner.val;
            mem::drop(node_inner);
            
            self.touch(node_clone);
        }
        res
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key) {
            let node_clone = node.clone();
            mem::drop(node);
            
            let mut node_mut = node_clone.borrow_mut();
            node_mut.val = value;
            mem::drop(node_mut);
                
            self.touch(node_clone);
        } else {
            if self.length == self.capacity {
                self.pop();
            }

            let node = new_rc_node(key, value);
            self.push(node.clone());
            self.map.insert(key, node);
        }
    }

    fn touch(&mut self, node: Rc<RefCell<ListNode>>) {
        let mut node_mut = node.borrow_mut();
        if node_mut.next.as_ref().unwrap().borrow().key == -1 {
            return;
        }

        // set node's prev's next to node's next
        // set node's next's prev to node's prev
        self.detach(node_mut);

        // set last's next to node
        let mut tail_mut = self.tail.borrow_mut();
        let last = tail_mut.prev.clone().expect("tail has no prev");
        let mut last_mut = last.borrow_mut();
        last_mut.next = Some(node.clone());
        mem::drop(last_mut);

        // update node's neighbors
        let mut node_mut = node.borrow_mut();
        node_mut.prev = Some(last);
        node_mut.next = Some(self.tail.clone());
        mem::drop(node_mut);

        // set tail's prev to node
        tail_mut.prev = Some(node);
    }

    fn detach(&mut self, node: RefMut<'_, ListNode>) {
        match (&node.prev, &node.next) {
            (None, None) => {},
            (Some(prev), Some(next)) => {
                let mut prev_mut = prev.borrow_mut();
                let mut next_mut = next.borrow_mut();
                prev_mut.next = Some(next.clone());
                next_mut.prev = Some(prev.clone());
            }
            _ => {/* unreachable!(); */}
        }
    }

    fn push(&mut self, node: Rc<RefCell<ListNode>>) {
        let mut tail_mut = self.tail.borrow_mut();

        // set last's next to node
        let last = tail_mut.prev.clone().expect("tail has no prev");
        let mut last_mut = last.borrow_mut();
        last_mut.next = Some(node.clone());
        mem::drop(last_mut);

        // set node's prev to last
        let mut node_mut = node.borrow_mut();
        node_mut.prev = Some(last);

        // set node's next to tail
        node_mut.next = Some(self.tail.clone());
        mem::drop(node_mut);

        // set tail's prev to node
        tail_mut.prev = Some(node);
        
        // increment length
        self.length += 1;
    }

    fn pop(&mut self) {
        let head_inner = self.head.borrow();
        let first = head_inner.next.clone().expect("head has no next");
        mem::drop(head_inner);

        // detach first
        self.detach(first.borrow_mut());

        // remove from map
        let first_inner = first.borrow();
        self.map.remove(&first_inner.key);
        mem::drop(first_inner);

        // decrement length
        self.length -= 1;
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */