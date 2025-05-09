impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut mag = [0_usize; 26];
        for c in magazine.chars() {
            mag[c as usize - 'a' as usize] += 1;
        }
        for c in ransom_note.chars() {
            let i = c as usize - 'a' as usize;
            if mag[i] == 0 {
                return false;
            }
            mag[i] -= 1;
        }
        true
    }
}