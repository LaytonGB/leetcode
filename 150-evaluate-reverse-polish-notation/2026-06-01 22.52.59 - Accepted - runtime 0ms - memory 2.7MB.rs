use std::str::FromStr;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());
        for s in tokens {
            match s.bytes().next_back().unwrap() {
                op @ b'+' | op @ b'-' | op @ b'*' | op @ b'/' => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(Self::apply(a, b, op));
                }
                _ => {
                    stack.push(i32::from_str(s.as_str()).unwrap());
                }
            }
        }
        stack[0]
    }

    fn apply(a: i32, b: i32, op: u8) -> i32 {
        match op {
            b'+' => a + b,
            b'-' => a - b,
            b'*' => a * b,
            b'/' => a / b,
            _ => unreachable!(),
        }
    }
}