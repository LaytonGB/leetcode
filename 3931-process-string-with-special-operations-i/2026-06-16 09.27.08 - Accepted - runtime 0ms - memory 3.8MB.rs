impl Solution {
    pub fn process_str(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            match c {
                '*' => { res.pop(); }
                '#' => res.extend_from_within(..),
                '%' => res = res.chars().rev().collect(),
                _ => { res.push(c); }
            }
        }
        res
    }
}