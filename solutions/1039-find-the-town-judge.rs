impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut pj = vec![true; n];
        let mut tc = vec![0; n];
        for v in trust.iter() {
            pj[v[0] as usize - 1] = false;
            tc[v[1] as usize - 1] += 1;
        }
        (0..n).find(|i| pj[*i] && tc[*i] == n - 1)
            .map(|i| i as i32 + 1)
            .unwrap_or(-1)
    }
}