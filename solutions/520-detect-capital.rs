impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut char_iter = word.chars();
        let (first, second) = (char_iter.next(), char_iter.next());
        if second.is_none() { return true; }
        let (first_cap, second_cap) = (first.unwrap().is_ascii_uppercase(), second.unwrap().is_ascii_uppercase());
        if !first_cap && second_cap { return false; }
        let check = if first_cap && second_cap {
            |c:&char| c.is_ascii_uppercase()
        } else {
            |c:&char| !c.is_ascii_uppercase()
        };
        for c in char_iter {
            if !check(&c) {
                return false;
            }
        }
        true
    }
}