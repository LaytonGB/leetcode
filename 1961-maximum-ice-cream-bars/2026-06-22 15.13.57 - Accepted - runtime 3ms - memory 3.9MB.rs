impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let n = costs.len();
        let max = costs.iter().max().unwrap().clone() as usize;

        let mut val_cnt: Vec<usize> = vec![0; max + 1];
        for &c in costs.iter() {
            val_cnt[c as usize] += 1;
        }
        for i in 1..=max {
            val_cnt[i] += val_cnt[i - 1];
        }

        let mut sorted = vec![0; n];
        for i in (0..n).rev() {
            val_cnt[costs[i] as usize] -= 1;
            sorted[val_cnt[costs[i] as usize]] = costs[i];
        }

        let mut res = 0;
        for cost in sorted {
            if coins < cost {
                break;
            }

            coins -= cost;
            res += 1;
        }
        res
    }
}