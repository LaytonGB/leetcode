impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars()
        .fold(Vec::new(), |mut stack, c| {
            if stack.last() == Some(&c) {
                stack.pop();
            } else {
                stack.push(c);
            }
            stack
        })
        .iter()
        .collect()
    }
}