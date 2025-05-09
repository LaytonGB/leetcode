use std::collections::*;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = Vec::new();
        Self::dfs(&nums, &mut res, &mut Vec::with_capacity(n), &mut vec![false; n]);
        res
    }

    fn dfs(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, visited: &mut Vec<bool>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        for i in 0..nums.len() {
            // skip if visited OR same as previous and previous not visited
            // this avoids duplicate patterns where same vals are pulled from diff idxs
            if visited[i] || i > 0 && nums[i] == nums[i-1] && !visited[i-1] {
                continue;
            }

            visited[i] = true;
            path.push(nums[i]);
            Self::dfs(nums, res, path, visited);
            path.pop();
            visited[i] = false;
        }
    }
}