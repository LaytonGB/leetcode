use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<String> = word_dict.into_iter().collect();
        let longest_word_len = words.iter().map(|w| w.len()).max().unwrap();
        
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=s.len() {
            for j in (i.checked_sub(longest_word_len).unwrap_or(0)..i).rev() {
                if dp[j] {
                    if words.contains(&s[j..i]) {
                        dp[i] = true;
                        break;
                    }
                }
            }
        }

        *dp.last().unwrap()
    }
}