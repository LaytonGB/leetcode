impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // track maximum reach and stop if bypassed
        let target = nums.len() - 1;
        let mut reach = 0;
        let mut max;
        for (i,n) in nums.into_iter().enumerate() {
            if i > reach { break; }
            max = i + n as usize;
            if max > reach {
                reach = max;
            }
            if reach >= target {
                return true;
            }
        }
        false
    }
}