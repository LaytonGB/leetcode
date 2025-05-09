impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();

        let mut res = 0;
        for i in 0..k {
            if let Some(h) = happiness.pop() {
                res += 0.max(h as i64 - i as i64);
            }
        }

        res
    }
}