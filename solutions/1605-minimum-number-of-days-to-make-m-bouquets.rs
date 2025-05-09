// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/solutions/686316/java-c-python-binary-search
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as usize * k as usize > bloom_day.len() {
            return -1;
        }

        let (mut l, mut h) = (1, *bloom_day.iter().max().unwrap());
        while l < h {
            let mid = (l + h) / 2;

            let mut bloomed_streak = 0;
            let mut bouquets_count = 0;
            for &x in bloom_day.iter() {
                bloomed_streak = if x > mid {
                    0
                } else {
                    bloomed_streak + 1
                };

                if bloomed_streak >= k {
                    bloomed_streak = 0;
                    bouquets_count += 1;
                    
                    if bouquets_count == m {
                        break;
                    }
                }
            }

            if bouquets_count == m {
                h = mid;
            } else {
                l = mid + 1;
            }
        }

        l
    }
}