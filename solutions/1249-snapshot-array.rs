use std::collections::*;
struct SnapshotArray {
    arr: Vec<Vec<(i32, i32)>>,
    snap_count: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self {
            arr: vec![vec![(0, 0)]; length as usize],
            snap_count: 0,
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        if self.arr[index as usize].last().unwrap().0 == self.snap_count {
            *self.arr[index as usize].last_mut().unwrap() = (self.snap_count, val);
        } else {
            self.arr[index as usize].push((self.snap_count, val));
        }
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_count += 1;
        self.snap_count - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.arr[index as usize].binary_search_by_key(&snap_id, |&(a, b)| a) {
            Ok(idx) => {
                self.arr[index as usize][idx].1
            }
            Err(idx) => {
                self.arr[index as usize][idx - 1].1
            }
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */