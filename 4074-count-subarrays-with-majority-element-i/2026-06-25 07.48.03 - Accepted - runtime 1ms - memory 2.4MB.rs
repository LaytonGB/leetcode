impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        
        // Index offset by +1
        // 0th value is sentinel
        let mut prefix = vec![[0, 0]; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            let [target_count, non_target_count] = prefix[i];
            prefix[i + 1] = if x == target {
                [target_count + 1, non_target_count]
            } else {
                [target_count, non_target_count + 1]
            };
        }

        let mut res = 0;
        for l in 0..n {
            let start_counts = prefix[l];
            for r in l..n {
                let end_counts = prefix[r + 1];
                let [target_count, non_target_count] = [
                    end_counts[0] - start_counts[0],
                    end_counts[1] - start_counts[1],
                ];
                if target_count > non_target_count {
                    res += 1;
                }
            }
        }

        res
    }
}