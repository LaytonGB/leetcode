impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let s = s.as_bytes();
        if *s.last().unwrap() != b'0' {
            return false;
        }
        
        let n = s.len();
        let (min, max) = (min_jump as usize, max_jump as usize);
        let mut vis = vec![false; s.len()];
        vis[0] = true;
        let mut q = vec![0];
        while let Some(i) = q.pop() {
            if i == n - 1 {
                return true;
            }

            for j in i + min..=(i + max).min(n - 1) {                
                if vis[j] {
                    continue;
                }
                vis[j] = true;
                
                if s[j] == b'0' {
                    q.push(j);
                }
            }
        }

        false
    }
}