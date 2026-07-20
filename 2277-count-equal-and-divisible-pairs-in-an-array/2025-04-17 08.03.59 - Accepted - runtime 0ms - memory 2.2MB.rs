use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;

        let mut indeces: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..n {
            indeces.entry(nums[i]).or_default().push(i);
        }

        let mut res = 0;
        for (_, indeces) in indeces.into_iter() {
            let n = indeces.len();
            for i in 0..n - 1 {
                for j in i + 1..n {
                    if (indeces[i] * indeces[j]) % k == 0 {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}