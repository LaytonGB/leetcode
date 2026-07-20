impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut last_pos: [i32; 3] = [-1; 3];
        s.bytes().enumerate().fold(0, |res, (i, b)| {
            last_pos[(b - b'a') as usize] = i as i32;
            res + 1 + last_pos.iter().min().unwrap()
        })
    }
}