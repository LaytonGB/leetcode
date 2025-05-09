impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0_i32; n];
        let mut q = Vec::new();
        for i in (0..n).rev() {
            while q.last().is_some_and(|&j| temperatures[i] >= temperatures[j]) {
                q.pop();
            }
            if !q.is_empty() {
                res[i] = (*q.last().unwrap() - i) as i32;
            }
            q.push(i);
        }
        res
    }
}