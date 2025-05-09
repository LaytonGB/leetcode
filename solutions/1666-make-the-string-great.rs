use std::str;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s = s.bytes().collect::<Vec<_>>();
        'outer: loop {
            for i in 0..s.len().checked_sub(1).unwrap_or(0) {
                if (s[i] < b'a' - 1 && s[i + 1] == s[i] + b'a' - b'A')
                || s[i + 1] == s[i] + b'A' - b'a'
                {
                    s.remove(i);
                    s.remove(i);
                    continue 'outer;
                }
            }
            break;
        }
        str::from_utf8(&s).unwrap().to_string()
    }
}