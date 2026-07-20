impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut res = 0;
        for p in patterns {
            if word.contains(&p) {
                res += 1;
            }
        }
        res
    }
}