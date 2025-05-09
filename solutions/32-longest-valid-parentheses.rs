use std::collections::*;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut stack = Vec::new();
        let mut res = 0;
        for (i,&c) in s.iter().enumerate() {
            if c == b'(' {
                stack.push(i);
            } else if !stack.is_empty() && s[*stack.last().unwrap()] == b'(' {
                stack.pop();
            } else {
                stack.push(i);
            }
        }

        if stack.is_empty() {
            res = n;
        } else {
            let mut a = n;
            while let Some(i) = stack.pop() {
                res = res.max(a - i - 1);
                a = i;
            }
            res = res.max(a);
        }

        res as i32
    }
}