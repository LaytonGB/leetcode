impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        
        // Index offset by +1
        // 0th value is sentinel
        let mut prefix = vec![0; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + if x == target { 1 } else { -1 };
        }

        let mut res = 0;
        for l in 0..n {
            for r in l..n {
                if prefix[r + 1] - prefix[l] > 0 {
                    res += 1;
                }
            }
        }

        res
    }
}