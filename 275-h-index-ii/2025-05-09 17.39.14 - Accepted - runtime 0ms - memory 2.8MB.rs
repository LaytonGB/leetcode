use std::cmp::Ordering as O;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let (mut l, mut h) = (0, n - 1);
        while l < h {
            let m = (l + h) / 2;
            if n - m > citations[m] as usize {
                l = m + 1;
            } else {
                h = m;
            }
        }

        if l < n && n - l <= citations[l] as usize {
            (n - l) as i32
        } else {
            0
        }
    }
}