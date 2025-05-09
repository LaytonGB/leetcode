impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut last_color, mut last_cost) = ('.', 0);

        for (c, &t) in colors.chars().zip(needed_time.iter()) {
            if c == last_color {
                res += t.min(last_cost);
                last_cost = t.max(last_cost);
            } else {
                last_color = c;
                last_cost = t;
            }
        }

        res
    }
}