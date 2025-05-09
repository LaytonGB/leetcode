use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(mut s: String) -> bool {
        let vowels = [b'a',b'e',b'i',b'o',b'u',b'A',b'E',b'I',b'O',b'U'].iter().collect::<HashSet<&u8>>();
        let t = s.split_off(s.len() / 2);
        // println!("{:?} {:?}", s, t);
        let (mut count_s, mut count_t) = (0,0);
        for c in s.as_bytes() {
            if vowels.contains(&c) {
                count_s += 1;
            }
        }
        for c in t.as_bytes() {
            if vowels.contains(&c) {
                count_t += 1;
            }
        }
        count_s == count_t
    }
}