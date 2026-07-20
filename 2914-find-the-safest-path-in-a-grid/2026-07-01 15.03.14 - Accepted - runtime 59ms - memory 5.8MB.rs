use std::collections::{BinaryHeap, VecDeque};

const ADJ: [usize; 5] = [0, 1, 0, !0, 0];

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        if m == 1 && n == 1 {
            return 0;
        }
        
        // --- Pre-process grid
        let mut vis = vec![false; m * n];
        let mut q = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    vis[i * m + j] = true;
                    q.push_back((i, j));
                }
            }
        }

        while !q.is_empty() {
            let q_len = q.len();
            for _ in 0..q_len {
                let (i, j) = q.pop_front().unwrap();
                let safeness = grid[i][j];
                for w in ADJ.windows(2) {
                    let (i2, j2) = (i + w[0], j + w[1]);
                    if i2 < m && j2 < n && !vis[i2 * m + j2] {
                        vis[i2 * m + j2] = true;
                        grid[i2][j2] = safeness + 1;
                        q.push_back((i2, j2));
                    }
                }
            }
        }
        
        // --- A* prioritise high value paths
        vis = vec![false; m * n];
        let mut h = BinaryHeap::new();
        h.push((grid[0][0], 0, 0));
        
        loop {
            let (min_safeness, i, j) = h.pop().unwrap();
            for w in ADJ.windows(2) {
                let (i2, j2) = (i + w[0], j + w[1]);

                if i2 == m - 1 && j2 == n - 1 {
                    return min_safeness.min(grid[i2][j2]) - 1;
                }
                
                if i2 < m && j2 < n && !vis[i2 * m + j2] {
                    vis[i2 * m + j2] = true;
                    h.push((
                        min_safeness.min(grid[i2][j2]),
                        i2,
                        j2
                    ));
                }
            }
        }

        unreachable!()
    }
}