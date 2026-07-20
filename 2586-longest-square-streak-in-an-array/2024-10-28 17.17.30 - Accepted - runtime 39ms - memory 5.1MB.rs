use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut map = HashMap::new();
        let mut res = 0;
        for n in nums {
            let streak = *map.get(&n).unwrap_or(&0);
            map.insert(n * n, streak + 1);
            res = res.max(streak + 1);
        }

        if res >= 2 {
            res
        } else {
            -1
        }
    }
}