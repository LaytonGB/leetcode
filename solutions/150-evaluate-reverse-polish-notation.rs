impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for s in tokens {
            match s.as_str() {
                "+" => Self::add(&mut stack),
                "-" => Self::sub(&mut stack),
                "*" => Self::mul(&mut stack),
                "/" => Self::div(&mut stack),
                _ => {
                    let n = s.parse::<i32>().unwrap();
                    stack.push(n);
                }
            }
        }
        stack.pop().unwrap() as i32
    }

    fn add(stack: &mut Vec<i32>) {
        let (n, m) = Self::pop2(stack);
        stack.push(m + n);
    }

    fn sub(stack: &mut Vec<i32>) {
        let (n, m) = Self::pop2(stack);
        stack.push(m - n);
    }

    fn mul(stack: &mut Vec<i32>) {
        let (n, m) = Self::pop2(stack);
        stack.push(m * n);
    }

    fn div(stack: &mut Vec<i32>) {
        let (n, m) = Self::pop2(stack);
        stack.push(m / n);
    }

    fn pop2(stack: &mut Vec<i32>) -> (i32, i32) {
        (stack.pop().unwrap(), stack.pop().unwrap())
    }
}