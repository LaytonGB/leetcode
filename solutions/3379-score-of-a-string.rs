impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes().windows(2).fold(0_i32, |res, w| res + w[0].abs_diff(w[1]) as i32)
    }
}