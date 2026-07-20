impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize;
        let mut diff_streak = 0;
        let mut res = 0;

        for i in 1..n + k - 1 {
            if colors[(i - 1) % n] == colors[i % n] {
                diff_streak = 0;
            } else {
                diff_streak += 1;
            }

            if diff_streak >= k - 1 {
                res += 1;
            }

            if i >= n && diff_streak == 0 {
                break;
            }
        }

        res
    }
}