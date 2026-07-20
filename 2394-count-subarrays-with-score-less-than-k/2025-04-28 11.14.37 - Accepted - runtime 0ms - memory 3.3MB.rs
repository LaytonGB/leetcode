impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let k = k as usize;
        let mut sum = 0;
        let mut res = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            sum += nums[i] as usize;
            while sum * (i - j + 1) >= k {
                sum -= nums[j] as usize;
                j += 1;
            }
            res += i - j + 1;
        }
        res as i64
    }
}