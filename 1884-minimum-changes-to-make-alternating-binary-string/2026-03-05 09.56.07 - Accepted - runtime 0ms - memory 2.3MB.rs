impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let (mut a, mut b) = (0, 0);
        let mut a_next = b'0';
        for c in s.bytes() {
            if c == a_next {
                b += 1;
            } else {
                a += 1;
            }
            if a_next == b'0' {
                a_next = b'1';
            } else {
                a_next = b'0';
            }
        }
        a.min(b)
    }
}