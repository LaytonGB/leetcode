impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let is_usable: Vec<bool> = nums.iter().map(|x| *x % modulo == k).collect();
        let mut count = 0;
        let mut res = 0;
        for l in 0..nums.len() {
            count = 0;
            for r in l..nums.len() {
                if is_usable[r] {
                    count += 1;
                }

                if count % modulo == k {
                    res += 1;
                }
            }
        }
        res
    }
}