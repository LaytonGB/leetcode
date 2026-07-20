// https://leetcode.com/problems/powx-n/solutions/19546/short-and-easy-to-understand-solution-by-j6qt

impl Solution {
    pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut exp = n as i64;

        Self::pow(x, exp)
    }

    fn pow(mut x: f64, mut exp: i64) -> f64 {
        if exp == 0 {
            return 1.0;
        }

        if exp < 0 {
            exp = -exp;
            x = 1_f64 / x;
        }

        if exp % 2 == 0 {
            Self::pow(x * x, exp / 2)
        } else {
            x * Self::pow(x * x, exp / 2)
        }
    }
}