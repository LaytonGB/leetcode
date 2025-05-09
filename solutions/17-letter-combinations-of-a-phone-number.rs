use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.chars().count() < 1 {
            return Vec::<String>::new();
        }
        let digit_map = HashMap::from([
            ('2', vec!["a", "b", "c"]),
            ('3', vec!["d", "e", "f"]),
            ('4', vec!["g", "h", "i"]),
            ('5', vec!["j", "k", "l"]),
            ('6', vec!["m", "n", "o"]),
            ('7', vec!["p", "q", "r", "s"]),
            ('8', vec!["t", "u", "v"]),
            ('9', vec!["w", "x", "y", "z"]),
        ]);
        let mut out = VecDeque::<String>::with_capacity(digits.chars().count() * 4);
        let mut last_out = VecDeque::<String>::with_capacity(digits.chars().count() * 4);
        last_out.push_back("".to_string());
        for d in digits.chars() {
            for s in &last_out {
                for c in &digit_map[&d] {
                    out.push_back(format!("{}{}", s, c));
                }
            }
            last_out = VecDeque::from_iter(out.drain(0..).map(|s| s.to_owned()));
        }
        last_out.into_iter().map(|s| s.to_owned()).collect()
    }
}