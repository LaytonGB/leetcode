impl Solution {
    pub fn is_power_of_four(mut n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && (n - 1) % 3 == 0
    }
}