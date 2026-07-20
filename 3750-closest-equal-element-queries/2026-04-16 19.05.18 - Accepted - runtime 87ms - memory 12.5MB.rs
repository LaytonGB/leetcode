impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        
        // Map each num to its last index
        let mut last_seen = [usize::MAX; 1_000_001];

        // Store the best distances
        let mut dist = vec![usize::MAX; n];

        // Iterate 2n to account for cyclic array
        for i in 0..n*2 {
            let curr_index = i % n;
            let x = nums[curr_index] as usize;
            if last_seen[x] != usize::MAX {
                let prev_index = last_seen[x];
                dist[curr_index] = dist[curr_index].min(i - last_seen[x]);
                dist[prev_index] = dist[prev_index].min(i - last_seen[x]);
            }
            last_seen[x] = curr_index;
        }

        // Map each query to its best distance or -1 if it mapped to itself after a cycle
        queries.into_iter()
            .map(|q| q as usize)
            .map(|q| {
                if dist[q] < n {
                    dist[q] as i32
                } else {
                    -1
                }
            })
            .collect()
    }
}