impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut res = [0; 37];
        let mut max = 0;
        for x in 1..=n as usize {
            let i = Self::to_idx(x);
            res[i] += 1;
            max = max.max(res[i]);
        }
        res.into_iter()
            .filter(|x| *x == max)
            .count() as i32
    }

    fn to_idx(n: usize) -> usize {
        let mut res = 0;
        for i in 0..=n.ilog10() {
            res += (n / 10_usize.pow(i)) % 10;
        }
        res
    }
}