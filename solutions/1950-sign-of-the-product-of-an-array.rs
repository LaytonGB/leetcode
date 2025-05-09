impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negative = false;
        for n in nums.iter() {
            if *n == 0 {
                return 0;
            } else if *n < 0 {
                negative = !negative;
            }
        }
        if negative {
            -1
        } else {
            1
        }
    }
}