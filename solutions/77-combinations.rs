impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut res = Vec::new();
        let mut i = 0_usize;
        let mut p = vec![0; k];
        while i < k {
            p[i] += 1;
            if p[i] > n { i -= 1; }
            else if i == k - 1 { res.push(p.clone()); }
            else {
                i += 1;
                p[i] = p[i - 1];
            }
        }
        res
    }
}