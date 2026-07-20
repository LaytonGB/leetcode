impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let (min, max) = nums.into_iter()
            .fold((i32::MAX, 0), |(min, max), x| {
                (min.min(x), max.max(x))
            });
        
        (max - min) as i64 * k as i64
    }
}