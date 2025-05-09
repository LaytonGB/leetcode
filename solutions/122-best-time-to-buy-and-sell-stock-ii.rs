impl Solution {
    pub fn max_profit(p: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..p.len() {
            if p[i - 1] < p[i] {
                res += p[i] - p[i - 1];
            }
        }
        res
    }
}