impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let mut res = 0_i32;
        for i in 0..=nums.len() as i32 {
            res ^= i;
        }
        for x in nums {
            res ^= x;
        }
        res
    }
}