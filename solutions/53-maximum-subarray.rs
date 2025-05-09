impl Solution {
    pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            nums[i] = nums[i] + if nums[i-1] > 0 { nums[i-1] } else { 0 };
        }
        nums.into_iter().max().unwrap()
    }
}