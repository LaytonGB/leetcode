use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let mut max_num = *nums.iter().max().unwrap();

        let mut res = 0;
        let mut count = 0;
        let mut l = 0;
        for h in 0..n {
            if nums[h] == max_num {
                count += 1;
            }

            while count >= k {
                if nums[l] == max_num {
                    count -= 1;
                }

                l += 1;
                res += n - h;
            }
        }

        res as i64
    }
}