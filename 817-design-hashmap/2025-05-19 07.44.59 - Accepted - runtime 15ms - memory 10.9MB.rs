const N: usize = 1_000_001;

struct MyHashMap([i32; N]);


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        MyHashMap([-1; N])
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.0[key as usize] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        self.0[key as usize]
    }
    
    fn remove(&mut self, key: i32) {
        self.0[key as usize] = -1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */