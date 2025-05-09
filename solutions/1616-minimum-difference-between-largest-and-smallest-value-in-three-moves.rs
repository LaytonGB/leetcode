// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/solutions/5406010/3-solutions-tutorial-o-n-included-minimum-difference-between-largest-and-smallest-value
impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        
        nums.sort_unstable();
        let mut res = *nums.last().unwrap() - nums[0];
        for i in 0..4 {
            res = res.min(nums[nums.len() - 4 + i] - nums[i]);
        }

        res
    }
}