impl Solution {
    pub fn min_zero_array(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut non_zero_count = nums.iter().filter(|n| **n != 0).count();

        if non_zero_count == 0 {
            return 0;
        }
        
        for (i, q) in queries.into_iter().enumerate() {
            let [l, r, val] = &q[..] else {
                unreachable!();
            };
            let (l, r) = (*l as usize, *r as usize);

            nums[l..=r].iter_mut().for_each(|n| {
                match n {
                    1.. if *n <= *val => {
                        *n = 0;
                        non_zero_count -= 1;
                    }
                    1.. => *n -= val,
                    _ => {}
                }
            });

            if non_zero_count == 0 {
                return i as i32 + 1;
            }
        }

        -1
    }
}