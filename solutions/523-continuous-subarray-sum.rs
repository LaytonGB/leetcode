use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::from([(0, -1)]);
        let mut sum = 0;
        for (i, x) in nums.iter().enumerate() {
            sum = (sum + x) % k;
            if let Some(j) = map.get(&sum) {
                if i as i32 - j >= 2 {
                    return true;
                }
            } else {
                map.insert(sum, i as i32);
            }
        }
        false
    }
}