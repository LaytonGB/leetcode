impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut res = 0;
        let mut num = 0;
        let mut sign = 1;
        for c in s.chars() {
            if c.is_ascii_digit() {
                num = 10 * num + c.to_digit(10).unwrap() as i32;
            } else if c == '+' {
                res += sign * num;
                num = 0;
                sign = 1;
            } else if c == '-' {
                res += sign * num;
                num = 0;
                sign = -1;
            } else if c == '(' {
                stack.push(res);
                stack.push(sign);
                sign = 1;
                res = 0;
            } else if c == ')' {
                res += sign * num;
                num = 0;
                res *= stack.pop().unwrap();
                res += stack.pop().unwrap();
            }
        }
        if num != 0 {
            res += sign * num;
        }
        res
    }
}