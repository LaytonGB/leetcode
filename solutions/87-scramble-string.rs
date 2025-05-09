use std::collections::HashMap;
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut history = HashMap::new();
        Self::solve(&mut history, &s1[..], &s2[..])
    }

    fn solve<'a>(history: &mut HashMap<(&'a str, &'a str), bool>, s1: &'a str, s2: &'a str) -> bool {
        let key = (s1.min(s2), s1.max(s2));
        if let Some(val) = history.get(&key) {
            return *val;
        }
        if s1 == s2 {
            return true;
        }
        let n = s1.len();
        if n != s2.len() {
            return false;
        }
        let mut chars = vec![0; 26];
        for (b1, b2) in s1.bytes().zip(s2.bytes()) {
            chars[(b1 - b'a') as usize] += 1;
            chars[(b2 - b'a') as usize] -= 1;
        }
        if !chars.into_iter().all(|n| n == 0) {
            return false;
        }
        for i in 1..n {
            // if scramble of first i elements is possible and ith letters match
            // or both strings at point are same length and letters match
            if Self::solve(history, &s1[..i], &s2[..i]) && Self::solve(history, &s1[i..], &s2[i..])
            || Self::solve(history, &s1[..i], &s2[n - i..]) && Self::solve(history, &s1[i..], &s2[..n - i]) {
                history.insert(key, true);
                return true;
            }
        }
        history.insert(key, false);
        false
    }
}