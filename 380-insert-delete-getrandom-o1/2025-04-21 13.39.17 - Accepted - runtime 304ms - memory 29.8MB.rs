use std::collections::HashSet;

struct RandomizedSet {
    set: HashSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        self.set.insert(val)
    }
    
    fn remove(&mut self, val: i32) -> bool {
        self.set.remove(&val)
    }
    
    fn get_random(&mut self) -> i32 {
        self.set = HashSet::from_iter(self.set.iter().copied());
        *self.set.iter().next().unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */