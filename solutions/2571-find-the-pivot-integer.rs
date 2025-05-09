use std::cmp::Ordering as O;
use std::collections::HashMap;
impl Solution {
    pub fn pivot_integer(mut n: i32) -> i32 {
        let x = ((n * (n + 1)) as f64 / 2.0).sqrt();
        if x % 1.0 == 0.0 {
            x as i32
        } else {
            -1
        }
    }
}