impl Solution {
    pub fn divide(a: i32, b: i32) -> i32 {
        // catch easy answers
        if a == b {
            return 1;
        }
        // store sign
        let is_pos = (a < 0) == (b < 0);
        // make positive
        let (mut a, b) = (a.abs() as u32, b.abs() as u32);
        // store 
        let mut res = 0;
        while a >= b {
            let mut n = 0;
            while a > b << (n + 1) {
                n += 1;
            }
            res += 1 << n;
            a -= b << n;
        }
        // catch overflows
        if res == 1 << 31 && is_pos {
            return i32::MAX;
        }
        // apply sign to result
        if is_pos {
            res as i32
        } else {
            -(res as i32)
        }
    }
}