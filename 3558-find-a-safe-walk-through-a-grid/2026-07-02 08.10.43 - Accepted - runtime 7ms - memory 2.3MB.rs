use std::collections::BinaryHeap;

const ADJ: [usize; 5] = [0, 1, 0, !0, 0];

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        if m == 1 && n == 1 {
            return health > grid[0][0];
        }

        let mut h = BinaryHeap::new();
        h.push((health - grid[0][0], 0, 0));
        let mut vis = vec![false; m * n];
        vis[0] = true;
        while let Some((health, i, j)) = h.pop() {
            for w in ADJ.windows(2) {
                let (i2, j2) = (i + w[0], j + w[1]);

                if i2 < m && j2 < n && !vis[i2 * n + j2] {
                    if i2 == m - 1 && j2 == n - 1 {
                        return health > grid[i2][j2];
                    }

                    vis[i2 * n + j2] = true;

                    if health - grid[i2][j2] > 0 {
                        h.push((health - grid[i2][j2], i2, j2));
                    }
                }
            }
        }

        false
    }
}