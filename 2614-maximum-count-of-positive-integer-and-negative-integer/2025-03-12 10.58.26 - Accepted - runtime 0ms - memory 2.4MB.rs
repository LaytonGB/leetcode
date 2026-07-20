impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold([0, 0], |[pos, neg], n| [pos + (n > 0) as i32, neg + (n < 0) as i32])
            .into_iter()
            .max()
            .unwrap()
    }
}