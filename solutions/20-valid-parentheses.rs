impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match (c) {
                '(' | '{' | '[' => stack.push(c),
                ')' => if stack.pop().unwrap_or('<') != '(' { return false; },
                '}' => if stack.pop().unwrap_or('<') != '{' { return false; },
                ']' => if stack.pop().unwrap_or('<') != '[' { return false; },
                _ => unreachable!()
            }
        }
        stack.is_empty()
    }
}