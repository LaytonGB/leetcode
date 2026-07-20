impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        let mut max = 0;
        for x in nums {
            if x == 1 {
                curr += 1;
                if curr > max {
                    max = curr;
                }
            } else {
                curr = 0;
            }
        }
        max
    }
}