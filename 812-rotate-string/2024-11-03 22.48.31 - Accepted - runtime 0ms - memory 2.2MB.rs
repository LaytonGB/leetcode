impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let n = s.len();
        if n != goal.len() {
            return false;
        }
        
        let s = s.as_bytes();
        let g = goal.as_bytes();
        let (start, n) = (0, s.len());
        'outer: for i in 0..n {
            for j in 0..n {
                if s[j] != g[(i + j) % n] {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}