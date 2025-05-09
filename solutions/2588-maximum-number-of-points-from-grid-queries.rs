use std::collections::HashSet;

const DIR: [usize; 5] = [0, 1, 0, !1, 0];

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        
        queries.into_iter()
            .map(|max| {
                let mut q = vec![(0, 0)];
                let mut visited = HashSet::new();
                let mut res = 0;
                while let Some((r, c)) = q.pop() {
                    if visited.contains(&(r, c)) {
                        continue;
                    }
                    
                    visited.insert((r, c));
                    if max > grid[r][c] {
                        res += 1;

                        for w in DIR.windows(2) {
                            if r + w[0] >= 0 && r + w[0] < m
                            && c + w[1] >= 0 && c + w[1] < n {
                                q.push((r + w[0], c + w[1]));
                            }
                        }
                    }
                }
                res
            })
            .collect()
    }
}