use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut memo = HashSet::<&str>::new();
        let mut res = HashSet::<&str>::new();

        unsafe {
            for i in 0..s.len().checked_sub(9).unwrap_or(0) {
                let t = s.get_unchecked(i..i + 10);
                if memo.contains(t) {
                    res.insert(t);
                } else {
                    memo.insert(t);
                }
            }
        }

        res.into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}