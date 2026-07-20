impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let mut diff = (nums.last().unwrap() - nums.first().unwrap()).abs();
        for i in 1..nums.len() {
            let d = (nums[i] - nums[i - 1]).abs();
            if d > diff {
                diff = d;
            }
        }

        diff as i32
    }
}