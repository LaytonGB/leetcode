impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut res = Vec::new();
        Self::dfs(&candidates, 0, &mut Vec::new(), &mut res, target);
        res
    }

    fn dfs(nums: &Vec<i32>, start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, tgt: i32) {
        if tgt == 0 {
            res.push(path.clone());
            return;
        }

        for i in start..nums.partition_point(|&x| x <= tgt) {
            // ignore consecutive duplicates to avoid final duplicates
            // without this, [1, 1, 1, 2], 2 -> [1, 1], [1, 1], [2]
            // indexes:                      -> [0, 1], [1, 2], [3]
            // with this, it'll enter the first result that sums to tgt,
            // and then ignore consecutive numbers of the same result
            if i > start && nums[i] == nums[i-1] {
                continue;
            }
            path.push(nums[i]);
            Self::dfs(nums, i + 1, path, res, tgt - nums[i]);
            path.pop(); // backtracking
        }
    }
}