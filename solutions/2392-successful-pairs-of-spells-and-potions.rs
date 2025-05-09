use std::cmp::Ordering::*;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let n = potions.len();
        potions.sort_unstable();
        spells
            .iter()
            .map(|&s| {
                (n - potions.binary_search_by(|&p| {
                    match (s as i64 * p as i64).cmp(&success) {
                        Less => Less,
                        Equal => Greater,
                        Greater => Greater,
                    }
                }).unwrap_err()) as i32
            })
            .collect()
    }
}