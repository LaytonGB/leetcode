/*
The logic here is that any number of palindromes can be made if 2 conditions are met:
    1. We have enough characters
    2. The number of odd-count characters <= k

Hence, we return false if k is greater than the length, else return true if the count
of odd-counted characters is <= k.
*/
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