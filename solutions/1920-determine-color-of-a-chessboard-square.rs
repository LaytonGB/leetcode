impl Solution {
    pub fn square_is_white(c: String) -> bool {
        let c = c.as_bytes();
        (c[0] - b'a' + c[1] - b'1') % 2 != 0
    }
}