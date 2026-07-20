impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return nums.into_iter().min().unwrap();
        }

        let k = k as usize;
        let n = nums.len();

        let (mut l, mut h) = nums.iter().fold((i32::MAX, 0), |(min, max), n| (min.min(*n), max.max(*n)));
        while l < h {
            let m = ((l as i64 + h as i64) / 2) as i32;
            
            let mut hits = 0;
            let mut i = 0;
            while i < n {
                if nums[i] <= m {
                    hits += 1;
                    if hits >= k {
                        break;
                    }

                    i += 2;
                } else {
                    i += 1;
                }
            }

            if hits >= k {
                h = m;
            } else {
                l = m + 1;
            }
        }

        l
    }
}