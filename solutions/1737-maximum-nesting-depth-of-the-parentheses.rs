impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0_i32;
        let mut res = 0_i32;
        for c in s.chars() {
            match c {
                '(' => {
                    depth += 1;
                    res = res.max(depth);
                }
                ')' => depth -= 1,
                _ => (),
            }
        }
        res
    }
}