use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::cmp::Ordering;

struct MedianFinder {
    l: BinaryHeap<i32>,
    h: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        Self {
            l: BinaryHeap::new(),
            h: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        match (self.l.peek(), self.h.peek()) {
            (None, None) => self.l.push(num), // if neither
            (Some(_), None) => self.l.push(num), // if only low
            (None, Some(_)) => self.h.push(Reverse(num)), // if only high
            (Some(_), Some(Reverse(n))) if num >= *n => self.h.push(Reverse(num)), // if both and num >= high lowest
            (Some(_), Some(_)) => self.l.push(num), // if both but num < high lowest
        }
        match self.l.len().cmp(&self.h.len()) {
            Ordering::Greater => {
                if let Some(n) = self.l.pop() {
                    self.h.push(Reverse(n));
                }
            }
            Ordering::Less => {
                if let Some(Reverse(n)) = self.h.pop() {
                    self.l.push(n);
                }
            }
            Ordering::Equal => {}
        }
    }
    
    fn find_median(&self) -> f64 {
        match (self.l.peek(), self.h.peek()) {
            (None, None) => { 0.0 }
            (Some(n), None) => { *n as f64 }
            (None, Some(Reverse(n))) => { *n as f64 }
            (Some(l), Some(Reverse(h))) => {
                match self.l.len().cmp(&self.h.len()) {
                    Ordering::Equal => {
                        (l + h) as f64 / 2.0
                    }
                    Ordering::Greater => {
                        *l as f64
                    }
                    Ordering::Less => {
                        *h as f64
                    }
                }
            }
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */