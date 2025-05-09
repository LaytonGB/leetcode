use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut map = HashMap::new();
        let mut res = -1;
        for n in nums {
            // Odd integer cannot have integer square root
            if n % 2 == 1 {
                continue;
            }
            
            let sqrt = (n as f32).sqrt() as i32;
            let streak = if let Some(streak) = map.get(&sqrt) {
                res = res.max(*streak + 1);
                *streak
            } else {
                0
            };

            map.insert(sqrt, streak + 1);
        }

        res
    }
}