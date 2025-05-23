struct NumArray {
    acc: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        Self {
            acc: std::iter::once(0)
                .chain(nums.iter().scan(0, |acc,n| {
                    *acc += n;
                    Some(*acc)
            })).collect()
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.acc[right as usize + 1] - self.acc[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */