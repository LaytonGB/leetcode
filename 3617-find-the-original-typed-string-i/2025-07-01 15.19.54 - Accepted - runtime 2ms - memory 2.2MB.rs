impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut chars = word.chars();
        let mut a = chars.next().unwrap();
        let mut res = 1; // original string counts too!
        for b in chars {
            if a == b {
                res += 1;
            }
            a = b;
        }
        res
    }
}