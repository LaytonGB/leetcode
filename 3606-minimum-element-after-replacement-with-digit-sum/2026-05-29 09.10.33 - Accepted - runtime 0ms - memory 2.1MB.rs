impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut res = i32::MAX;
        for mut n in nums {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            res = res.min(sum);
        }
        res
    }
}