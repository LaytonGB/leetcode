use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    list: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            map: HashMap::new(),
            list: Vec::new(),
            rng: rand::thread_rng(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            let idx = self.list.len();
            self.map.insert(val, idx);
            self.list.push(val);
            true
        } else {
            false
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last = *self.list.last().unwrap();
            self.map.insert(last, idx);
            self.list.swap_remove(idx);
            self.map.remove(&val);
            true
        } else {
            false
        }
    }
    
    fn get_random(&mut self) -> i32 {
        let idx = self.rng.gen_range(0..self.list.len());
        self.list[idx]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */