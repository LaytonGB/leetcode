impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::r(&nums, 0, 0)
    }

    fn r(nums: &[i32], i: usize, sum: i32) -> i32 {
        if i == nums.len() {
            sum
        } else {
            Self::r(nums, i + 1, sum ^ nums[i])
            + Self::r(nums, i + 1, sum)
        }
    }
}