use std::collections::*;
use rand::rngs::ThreadRng;
use rand::Rng;
struct Solution {
    thread: ThreadRng,
    vals: Vec<i32>,
}
impl Solution {

    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut v = Vec::new();
        while let Some(h) = head {
            v.push(h.val);
            head = h.next;
        }
        Self {
            thread: rand::thread_rng(),
            vals: v,
        }
    }
    
    fn get_random(&mut self) -> i32 {
        if self.vals.len() == 1 {
            return self.vals[0];
        }
        let lim: usize = self.thread.gen_range(0, self.vals.len());
        for n in self.vals.iter().skip(lim) {
            return *n;
        }
        unreachable!()
    }
}
