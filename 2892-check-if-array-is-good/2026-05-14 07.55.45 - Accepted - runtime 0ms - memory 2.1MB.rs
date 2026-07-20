impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32 - 1;

        let mut seen_double_n = false;
        let mut seen = [false; 100];

        for x in nums {
            if x > n {
                return false;
            }
            
            if seen[(x - 1) as usize] {
                if x == n {
                    if seen_double_n {
                        return false;
                    }

                    seen_double_n = true;
                    continue;
                }

                return false;
            }

            seen[(x - 1) as usize] = true;
        }

        seen.into_iter()
            .take(n as usize)
            .all(|seen| seen)
            && seen_double_n
    }
}