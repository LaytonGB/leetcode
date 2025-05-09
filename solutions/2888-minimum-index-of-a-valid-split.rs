use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        let (dominant, count) = nums.iter()
            .fold(HashMap::new(), |mut m, x| {
                m.entry(*x)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                m
            })
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .unwrap();
        
        let mut rolling_count = 0;
        for (i, x) in nums.into_iter().enumerate() {
            if x == dominant {
                rolling_count += 1;

                if rolling_count > (i + 1) as i32 / 2 {
                    if count - rolling_count > (n - i - 1) as i32 / 2 {
                        return i as i32;
                    } else {
                        return -1;
                    }
                }
            }
        }

        unreachable!()
    }
}