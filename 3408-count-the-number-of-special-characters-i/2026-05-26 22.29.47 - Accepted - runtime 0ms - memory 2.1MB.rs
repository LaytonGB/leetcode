impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut upper = [false; 26];
        let mut lower = [false; 26];

        for b in word.bytes() {
            if b < 97 {
                upper[(b - b'A') as usize] = true;
            } else {
                lower[(b - b'a') as usize] = true;
            }
        }

        let mut res = 0;
        for (upper, lower) in upper.into_iter().zip(lower.into_iter()) {
            if upper && lower {
                res += 1;
            }
        }

        res
    }
}