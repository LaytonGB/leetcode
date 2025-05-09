impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a1 = [0_usize; 26];
        for c in s.into_bytes() {
            a1[(c - b'a') as usize] += 1;
        }
        let mut a2 = [0_usize; 26];
        for c in t.into_bytes() {
            a2[(c - b'a') as usize] += 1;
        }
        a1 == a2
    }
}