use std::collections::HashMap;
impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {        
        let mut res = 0;
        let mut counts = HashMap::with_capacity(nums.len());
        let mut l = 0;
        for h in 0..nums.len() {
            let num_h = nums[h];
            let new_count = *counts.entry(num_h)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            if new_count > k {
                loop {
                    let num_l = nums[l];
                    *counts.get_mut(&num_l).unwrap() -= 1;
                    l += 1;

                    if l >= h || num_h == num_l {
                        break;
                    }
                }
            }

            res = res.max(h - l + 1);
        }

        res as i32
    }
}