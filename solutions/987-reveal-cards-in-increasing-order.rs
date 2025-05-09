use std::collections::VecDeque;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();

        let mut indices = (0..deck.len()).collect::<VecDeque<_>>();
        let mut res = vec![0; deck.len()];
        for c in deck.into_iter() {
            let i = indices.pop_front().unwrap();
            res[i] = c;
            
            if !indices.is_empty() {
                indices.rotate_left(1);
            }
        }

        res
    }
}