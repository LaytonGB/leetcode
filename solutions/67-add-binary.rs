use std::iter::{once, repeat};

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // amazing answer from https://leetcode.com/problems/add-binary/solutions/1061502/rust-one-liner/
        format!("{:b}", u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap())
    }
}