use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::with_capacity(nums.len());
        let mut res = None;
        for (i, &x) in nums.iter().enumerate() {
            if let Some(j) = m.get(&x) {
                res = match res {
                    None => Some(i - j),
                    Some(res) => Some(res.min(i - j)),
                };
            }
            let r = Self::reverse(x);
            m.insert(r, i);
        }
        match res {
            None => -1,
            Some(res) => res as i32,
        }
    }

    fn reverse(mut x: i32) -> i32 {
        let mut res = 0;
        while x != 0 {
            res = res * 10 + x % 10;
            x /= 10;
        }
        res
    }
}