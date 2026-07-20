use std::collections::HashSet;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::ls(&s, k as usize) as i32
    }

    fn ls(s: &str, k: usize) -> usize {
        if s.len() < k {
            return 0;
        }
        
        let set = HashSet::<char>::from_iter(s.chars());
        for &c in set.iter() {
            if s.chars().filter(|c2| c == *c2).count() < k {
                return s.split(c).map(|t| Self::ls(t, k)).max().unwrap();
            }
        }

        s.len()
    }
}