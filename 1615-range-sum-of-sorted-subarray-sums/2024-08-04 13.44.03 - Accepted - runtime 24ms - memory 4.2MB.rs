// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/solutions/5582910/easy-solution-beats-100-2-approaches
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![];
        let mut i = 0;
        while i < n {
            let mut prefix = 0;
            for j in i..n {
                prefix += nums[j];
                arr.push(prefix);
            }
            i += 1;
        }
        arr.sort_unstable();
        arr[left as usize - 1..right as usize]
            .iter()
            .copied()
            .reduce(|a, b| (a + b) % MOD)
            .unwrap()
    }
}