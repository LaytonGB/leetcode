impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let mut it = s.chars().rev();
        while let Some(c) = it.next() {
            match (c, res) {
                (' ', 1..) => break,
                (' ', _) => continue,
                _ => res += 1,
            }
        }
        res
    }
}