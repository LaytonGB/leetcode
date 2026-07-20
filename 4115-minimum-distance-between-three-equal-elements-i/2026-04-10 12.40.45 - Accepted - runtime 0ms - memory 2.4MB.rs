impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
        
        let mut nums_labelled = nums.into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        nums_labelled.sort_unstable_by_key(|x| (x.1, x.0));

        nums_labelled
            .windows(3)
            .fold(None, |res, w| {
                let (a, b, c) = (w[0], w[1], w[2]);
                if a.1 == c.1 {
                    let dist = (a.0.abs_diff(b.0) + b.0.abs_diff(c.0) + c.0.abs_diff(a.0)) as i32;
                    if res.is_none_or(|r| dist < r) {
                        return Some(dist);
                    }
                }
                res
            })
            .unwrap_or(-1)
    }
}