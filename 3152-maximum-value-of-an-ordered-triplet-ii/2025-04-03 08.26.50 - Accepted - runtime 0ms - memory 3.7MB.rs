impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let (mut res, mut max_a, mut max_a_sub_b) = (0_i64, 0_i64, 0_i64);
        for i in 0..nums.len() {
            res = res.max(max_a_sub_b * nums[i] as i64);
            max_a_sub_b = max_a_sub_b.max(max_a - nums[i] as i64);
            max_a = max_a.max(nums[i] as i64);
        }
        res
    }
}