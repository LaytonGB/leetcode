impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let n = nums.len();
        
        let (mut max_i, mut min_i) = (-1, -1);
        let mut l = 0;
        let mut res = 0_i64;
        for r in 0..n {
            let x = nums[r];

            if x < min_k || x > max_k {
                l = r + 1;
                continue;
            }

            if x == max_k {
                max_i = r as i32;
            }
            if x == min_k {
                min_i = r as i32;
            }

            res += 0.max(min_i.min(max_i) - l as i32 + 1) as i64;
        }

        res
    }
}