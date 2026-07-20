// https://leetcode.com/problems/count-subarrays-with-majority-element-i/solutions/8356776/3-approaches-explained-brute-force-prefi-g8st

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        
        let mut prefix = vec![0; 2 * n + 1];
        prefix[n] = 1;

        let mut count = n;
        let mut presum = 0;
        let mut res = 0;
        for &x in nums.iter() {
            if x == target {
                presum += prefix[count];
                count += 1;
            } else {
                count -= 1;
                presum -= prefix[count];
            }
            prefix[count] += 1;
            res += presum;
        }

        res
    }
}