impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words.into_iter()
            .map(|w| Self::word_to_weight(w, &weights))
            .map(|w| Self::weight_to_char(w))
            .collect()
    }

    fn word_to_weight(w: String, weights: &Vec<i32>) -> i32 {
        w.bytes()
            .map(|b| weights[(b - b'a') as usize])
            .sum()
    }

    fn weight_to_char(w: i32) -> char {
        ((25 - (w % 26) as u8) + b'a') as char
    }
}