impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        Self::r(1, 1, n).unwrap() + 1
    }

    fn r(s: i32, c: i32, t: i32) -> Option<i32> {
        if s == t {
            return Some(0);
        } else if s > t {
            return None;
        }

        match (Self::r(s + c, c, t), Self::r(s * 2, s, t)) {
            (Some(a), Some(b)) => Some((a + 1).min(b + 2)),
            (Some(a), None) => Some(a + 1),
            (None, Some(b)) => Some(b + 2),
            _ => None,
        }
    }
}