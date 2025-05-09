fn byte_to_idx(b: u8) -> usize {
    (b - b'a') as usize
}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut char_indeces = [-1; 3];
        let mut res = 0;

        for i in 0..s.len() {
            char_indeces[byte_to_idx(s[i])] = i as i32;
            res += 1 + char_indeces.iter().min().unwrap();
        }

        res
    }
}