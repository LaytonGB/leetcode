impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        
        let mut res = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                if nums[i] == nums[j] && (i * j) % k == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}