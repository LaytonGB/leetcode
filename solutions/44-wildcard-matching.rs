use std::cmp::Ordering as O;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        
        let (m, n) = (s.len(), p.len());
        let (mut i, mut j) = (0, 0);
        let mut asterisk_match_count = 0;
        let mut last_asterisk_index = None;

        while i < m {
            if j < n && (p[j] == b'?' || s[i] == p[j]) {
                i += 1;
                j += 1;

            } else if j < n && p[j] == b'*' {
                last_asterisk_index = Some(j);
                asterisk_match_count = i;
                j += 1;

            } else if let Some(k) = last_asterisk_index {
                j = k + 1;
                asterisk_match_count += 1;
                i = asterisk_match_count;

            } else {
                return false;
            }
        }

        (j..n).all(|j| p[j] == b'*')
    }
}
