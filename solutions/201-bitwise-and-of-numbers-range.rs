impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        for i in 0..=32 {
            if left == right {
                return left << i;
            }
            left >>= 1;
            right >>= 1;
        }
        0_i32
    }
}