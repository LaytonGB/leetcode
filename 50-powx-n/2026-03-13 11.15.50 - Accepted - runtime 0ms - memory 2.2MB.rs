// https://leetcode.com/problems/powx-n/solutions/3871230/rust-0ms-solution-exponentiation-by-squa-e01l

impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut exp = n as i64;
        if exp < 0 {
            exp = -exp;
            x = 1_f64 / x;
        }

        let mut res = 1.0;
        while exp > 0 {
            if exp % 2 == 1 {
                res *= x;
            }
            x *= x;
            exp /= 2;
        }

        res
    }
}