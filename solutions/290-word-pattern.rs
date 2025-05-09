use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let [mut p1, mut p2] = [0; 2];
        let mut m1 = HashMap::<char, u8>::new();
        let mut m2 = HashMap::<String, u8>::new();
        for (c,w) in pattern.chars().zip(s.split(" ")) {
            let has_c = m1.contains_key(&c);
            let has_w = m2.contains_key(w);
            if has_c {
                if !has_w {
                    return false;
                }
                
                let i1 = m1.get(&c).unwrap();
                let i2 = m2.get(w).unwrap();
                if i1 != i2 {
                    return false
                }
            } else {
                if has_w {
                    return false;
                }
                
                m1.insert(c, p1);
                p1 += 1;
                m2.insert(w.to_string(), p2);
                p2 += 1;
            }
        }
        pattern.len() == s.split(" ").count()
    }
}