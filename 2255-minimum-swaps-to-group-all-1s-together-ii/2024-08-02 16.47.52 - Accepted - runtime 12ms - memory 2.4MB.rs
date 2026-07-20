// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii/solutions/5571652/same-concept-of-sliding-window-o-n-in-3-langs-margin-94
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let k = nums.iter().filter(|x| **x == 1).count();
        let mut max: i32 = nums.iter().take(k).sum();
        let mut count = max;
        let n = nums.len();
        for i in k..n + k {
            count += nums[i % n];
            count -= nums[(i - k + n) % n];
            max = max.max(count);
        }
        k as i32 - max 
    }
}