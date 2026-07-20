impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let chars: Vec<char> = word.chars().collect();
        let mut res = 1; // original string counts too!
        for w in chars.windows(2) {
            if w[0] == w[1] {
                res += 1;
            }
        }
        res
    }
}