impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let f = |(res, curr): (i64, i64), n: &i32| {
            if *n == 0 {
                (res + curr + 1, curr + 1)
            } else {
                (res, 0)
            }
        };
        nums.iter().fold((0, 0), f).0
    }
}