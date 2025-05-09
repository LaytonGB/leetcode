use std::collections::*;
impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>, diff: i32) -> i32 {
        let n = nums.len();
        
        let map: HashMap<i32, Vec<usize>> = nums.iter()
            .enumerate()
            .fold(HashMap::new(), |mut m, (i,&x)| {
                m.entry(x - diff)
                    .and_modify(|v| v.push(i))
                    .or_insert(vec![i]);
                m
            });
        
        let mut visited = HashSet::new();
        (0..n).fold(0, |mut res, i| {
            if !visited.contains(&nums[i]) {
                res.max(
                    Self::dfs(&map, &nums, &mut visited, i)
                )
            } else {
                res
            }
        })
    }

    fn dfs(map: &HashMap<i32, Vec<usize>>, nums: &Vec<i32>, vis: &mut HashSet<i32>, idx: usize) -> i32 {
        vis.insert(nums[idx]);
        if let Some(next_idxs) = map.get(&nums[idx]) {
            let i = next_idxs.partition_point(|&x| x <= idx);
            if i < next_idxs.len() {
                return Self::dfs(map, nums, vis, next_idxs[i]) + 1;
            }
        }
        1
    }
}