use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut m = s.chars().fold(HashMap::with_capacity(s.len()), |mut m,c| {
            *m.entry(c).or_insert(0) += 1;
            m
        });
        for c in t.chars() {
            if let Some(e) = m.get_mut(&c) {
                if *e > 0 {
                    *e -= 1;
                } else {
                    return c;
                }
            } else {
                return c;
            }
        }
        'a'
    }
}