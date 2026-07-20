impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        
        1 + Self::find_min_steps(1, 1, n)
    }

    fn find_min_steps(curr: i32, clip: i32, tgt: i32) -> i32 {
        if curr == tgt {
            0
        } else if curr > tgt {
            i32::MAX / 2
        } else {
            let cp = 2 + Self::find_min_steps(curr * 2, curr, tgt);
            let p = 1 + Self::find_min_steps(curr + clip, clip, tgt);
            cp.min(p)
        }
    }
}