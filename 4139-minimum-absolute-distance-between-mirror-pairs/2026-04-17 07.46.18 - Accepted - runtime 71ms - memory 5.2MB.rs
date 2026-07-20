use std::collections::BTreeMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        let mut m = BTreeMap::new();
        let mut res = usize::MAX;
        for (i, &x) in nums.iter().enumerate() {
            if let Some(j) = m.get(&x) {
                res = res.min(i - j);
            }
            let r = Self::reverse(x);
            m.insert(r, i);
        }
        if res == usize::MAX {
            -1
        } else {
            res as i32
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