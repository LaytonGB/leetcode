impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize;
        let mut matching_pair_positions = vec![];
        
        for i in 0..n - 1 {
            if colors[i] == colors[i + 1] {
                matching_pair_positions.push(i);
            }
        }

        if colors[n - 1] == colors[0] {
            matching_pair_positions.push(n - 1);
        }

        if matching_pair_positions.is_empty() {
            return n as i32;
        }

        matching_pair_positions.push(matching_pair_positions[0] + n);

        let mut res = 0;
        for w in matching_pair_positions.windows(2) {
            let [i, j] = &w[..] else {
                unreachable!()
            };

            let diff = (j - i - k + 1) as i32;

            if diff > 0 {
                res += diff;
            }
        }

        res
    }
}