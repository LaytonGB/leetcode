// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/solutions/4811460/greedy-sacrifice
impl Solution {
    pub fn maximum_value_sum(mut nums: Vec<i32>, k: i32, mut edges: Vec<Vec<i32>>) -> i64 {
        let best_sum = nums.iter().map(|&x| x.max(x ^ k) as i64).sum::<i64>();
        let count = nums.iter().filter(|&x| x ^ k > *x).count() as i64;
        if count % 2 == 0 {
            best_sum
        } else {
            best_sum - nums.iter().map(|&x| x.abs_diff(x ^ k)).min().unwrap() as i64
        }
    }
}