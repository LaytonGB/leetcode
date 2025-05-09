use std::collections::*;
impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 { continue; }
                // println!("Zero group found at {} {}", i, j);
                let mut q = VecDeque::new();
                q.push_back((i, j));
                let mut isolated = true;
                while let Some((i, j)) = q.pop_front() {
                    if grid[i][j] == 1 { continue; }
                    // println!("Checking adj for {} {}", i, j);
                    for w in [0, 1, 0, !1 + 1, 0].windows(2) {
                        // println!("WINDOWS: {:?}", w);
                        let (r, c) = (i.overflowing_add(w[0]).0, j.overflowing_add(w[1]).0);
                        // println!("Checking: {} {}", r, c);
                        if r >= n || c >= m {
                            // println!("Dismissed coords {} {}", r, c);
                            isolated = false;
                            continue;
                        }
                        if grid[r][c] == 0 {
                            // println!("Queued coords {} {}", r, c);
                            q.push_back((r, c));
                        }
                    }
                    grid[i][j] = 1;
                }
                if isolated {
                    // println!("Group was isolated");
                    res += 1;
                }
            }
        }
        res
    }
}