use std::str::Chars;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut chars = s.chars();
        let mut res = String::new();
        while let Some(c) = chars.next() {
            match c {
                '(' => res.push_str(&Self::rev_inner(&mut chars)),
                c => res.push(c),
            }
        }
        res
    }

    fn rev_inner(s: &mut Chars) -> String {
        let mut res = String::new();
        while let Some(c) = s.next() {
            match c {
                '(' => res.push_str(&Self::rev_inner(s)),
                ')' => return res.chars().rev().collect(),
                c => res.push(c),
            }
        }
        unreachable!()
    }
}