impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = [0; 26];
        for b in s.bytes() {
            count[(b - b'a') as usize] += 1;
        }
        for (i,b) in s.bytes().enumerate() {
            if count[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}