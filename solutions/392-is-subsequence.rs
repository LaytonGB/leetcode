impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() { return false; }
        let mut iter = t.chars();
        'outer: for c1 in s.chars() {
            while let Some(c2) = iter.next() {
                if c1 == c2 { continue 'outer; }
            }
            return false;
        }
        true
    }
}