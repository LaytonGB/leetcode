impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut res = [0; 36];
        let mut max = 0;
        for x in 1..=n as usize {
            let i = Self::to_idx(x) - 1;
            res[i] += 1;
            max = max.max(res[i]);
        }
        res.into_iter()
            .filter(|x| *x == max)
            .count() as i32
    }

    fn to_idx(mut n: usize) -> usize {
        let mut res = 0;
        while n > 0 {
            res += n % 10;
            n /= 10;
        }
        res
    }
}