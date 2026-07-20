impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for mut x in nums.into_iter().rev() {
            while x > 0 {
                res.push(x % 10);
                x /= 10;
            }
        }
        res.into_iter().rev().collect()
    }
}