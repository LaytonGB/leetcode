impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if (s.len() < k as usize) {
            return false;
        }
        
        let odds_count = Self::get_odds_count(s);
        odds_count <= k
    }

    fn get_odds_count(s: String) -> i32 {
        s.bytes()
            .map(|b| (b - b'a') as usize)
            .fold([false; 26], |mut has_odd_counts, b| {
                has_odd_counts[b] = !has_odd_counts[b];
                has_odd_counts
            })
            .into_iter()
            .filter(|is_odd| *is_odd)
            .count() as i32
    }
}