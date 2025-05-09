impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut res = 0;
        let mut n = 0;
        for c in s.chars() {
            if c == '0' {
                res = n.min(res + 1)
            } else {
                n += 1
            }
        }
        res
    }
}