use std::collections::HashMap;
use std::iter::{once, repeat};

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let alpha = once((b'a' - 1) as char).chain(order.chars()).enumerate().fold(HashMap::new(), |mut m, (i, c)| {
            m.insert(c, i);
            m
        });      
        for w in words.windows(2) {
            let (w1, w2) = (&w[0], &w[1]);
            for (c1, c2) in w1.chars().zip(w2.chars().chain(repeat((b'a' - 1) as char))) {
                let (n1, n2) = (alpha.get(&c1).unwrap(), alpha.get(&c2).unwrap());
                if n1 > n2 {
                    return false;
                }
                if n1 < n2 {
                    break;
                }
            }
        }
        true
    }
}