impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        
        let mut res = 0;
        for (i, j, k) in (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))).flat_map(|(i, j)| (j + 1..n).map(move |k| (i, j, k))) {
            res = res.max((nums[i] as i64 - nums[j] as i64) * nums[k] as i64);
        }

        res
    }
}