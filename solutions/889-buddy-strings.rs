impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let mut diffs = 0;
        let mut s_char_presence = vec![false; 26];
        let mut s_has_duplicates = false;
        let mut diff_pairs = Vec::with_capacity(2);
        for (c1, c2) in s.bytes().zip(goal.bytes()) {
            if s_char_presence[(c1 - b'a') as usize] {
                s_has_duplicates = true;
            } else {
                s_char_presence[(c1 - b'a') as usize] = true;
            }
            if c1 ^ c2 != 0 {
                if diffs < 2 {
                    diffs += 1;
                    diff_pairs.push((c1, c2));
                } else {
                    return false;
                }
            }
        }

        match diffs {
            0 => s_has_duplicates,
            2 => diff_pairs[0].0 == diff_pairs[1].1 && diff_pairs[0].1 == diff_pairs[1].0,
            _ => false,
        }
    }
}