use std::collections::*;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut max_sub = s.iter().take(k)
            .fold(0, |n, c| if vowels.contains(c) {n + 1} else {n});
        let mut curr_sub = max_sub;
        for i in k .. s.len() {
            if vowels.contains(&s[i - k]) {
                curr_sub -= 1;
            }
            if vowels.contains(&s[i]) {
                curr_sub += 1;
            }
            max_sub = max_sub.max(curr_sub);
        }
        max_sub
    }
}