impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut res = 0;
        for i in 0..strs[0].len() {
            for w in strs.windows(2) {
                let (s1,s2) = (&w[0],&w[1]);
                if s1.as_bytes()[i] > s2.as_bytes()[i] {
                    res += 1;
                    break;
                }
            }
        }
        res
    }
}