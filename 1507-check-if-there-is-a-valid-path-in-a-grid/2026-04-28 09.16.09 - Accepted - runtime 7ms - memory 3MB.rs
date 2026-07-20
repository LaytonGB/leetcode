// https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/solutions/547371/java-clean-bfs-by-rexue70-31w3

use std::collections::LinkedList;

const DIRS: [[[usize; 2]; 2]; 6] = [
    [[0, !0], [0, 1]],
    [[!0, 0], [1, 0]],
    [[0, !0], [1, 0]],
    [[0, 1], [1, 0]],
    [[0, !0], [!0, 0]],
    [[0, 1], [!0, 0]],
];

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut vis = vec![vec![false; n]; m];
        vis[0][0] = true;
        let mut q = LinkedList::new();
        q.push_back([0, 0]);
        while let Some([x, y]) = q.pop_front() {
            let val = grid[x][y] - 1;
            for d in DIRS[val as usize] {
                // get next
                let (nx, ny) = (x + d[0], y + d[1]);
                
                // bounds check
                if nx >= m || ny >= n || vis[nx][ny] {
                    continue;
                }

                // ensure next cell leads back to this cell
                if !DIRS[(grid[nx][ny] - 1) as usize].iter().any(|&d| nx + d[0] == x && ny + d[1] == y) {
                    continue;
                }

                vis[nx][ny] = true;
                q.push_back([nx, ny]);
            }
        }

        // did reach last pos?
        vis[m-1][n-1]
    }
}