impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let idx = letters.partition_point(|&c| c <= target);
        if idx < letters.len() {
            letters[idx]
        } else {
            letters[0]
        }
    }
}