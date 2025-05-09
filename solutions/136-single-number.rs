impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut s_nums = nums.clone();
        s_nums.sort_unstable();
        let mut i = 0;
        while i < s_nums.len()-1 && s_nums[i] == s_nums[i+1] {
            i += 2;
        }
        s_nums[i]
    }
}