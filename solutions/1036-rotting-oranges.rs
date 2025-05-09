use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (grid.len(), grid[0].len());
        let mut mat = vec![vec![i32::MAX; n]; m];
        let mut q = VecDeque::new();
        let mut res = 0;
        for (i,row) in grid.iter().enumerate() {
            for (j,&n) in row.iter().enumerate() {
                if n == 2 {
                    mat[i][j] = 0;
                    q.push_back((i,j));
                }
            }
        }
        while let Some((i,j)) = q.pop_front() {
            for d in [0,1,0,!0,0].windows(2) {
                let (r,c) = (i+d[0], j+d[1]);
                let new_val = mat[i][j] + 1;
                if r<m && c<n && grid[r][c] == 1 && mat[r][c] > new_val {
                    mat[r][c] = new_val;
                    res = res.max(new_val);
                    q.push_back((r,c));
                }
            }
        }
        for (i,(r1,r2)) in mat.iter().zip(grid.iter()).enumerate() {
            for (j,(&n1,&n2)) in r1.iter().zip(r2.iter()).enumerate() {
                if n1 == i32::MAX && n2 == 1 {
                    return -1;
                }
            }
        }
        res
    }
}