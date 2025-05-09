use std::collections::VecDeque;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut count = [false; 255];
        let mut res = 0;
        let mut len = 0;
        let mut hist = VecDeque::with_capacity(s.len());
        for b in s.bytes() {
            let b = b as usize;
            while count[b] {
                let b = hist.pop_front().unwrap();
                count[b] = false;
                len -= 1;
            }
            hist.push_back(b);
            count[b] = true;
            len += 1;
            res = res.max(len);
        }
        res
    }
}