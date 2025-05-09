use std::collections::HashMap;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let position = order.bytes()
            .enumerate()
            .fold(vec![0; 26], |mut v, (i, b)| {
                v[(b - b'a') as usize] = i;
                v
            });
        
        let mut s = s.chars().collect::<Vec<char>>();
        s.sort_unstable_by_key(|c| position[(*c as u8 - b'a') as usize]);

        s.into_iter().collect()
    }
}