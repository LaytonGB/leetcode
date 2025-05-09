use std::borrow::BorrowMut;
use std::collections::VecDeque;

struct RecentCounter {
    v: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter{ v:VecDeque::new() }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        let b = self.v.borrow_mut();
        b.push_back(t);
        while let Some(&n) = b.front() {
            if t - n > 3000 {
                b.pop_front();
            } else {
                break;
            }
        }
        b.len() as i32
    }

}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */