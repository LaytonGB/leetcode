impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let (r, c, k) = (x as usize, y as usize, k as usize);
        
        for i in 0..k / 2 {
            for j in 0..k {
                let temp = grid[r+i][c+j];
                grid[r+i][c+j] = grid[r+k-i-1][c+j];
                grid[r+k-i-1][c+j] = temp;
            }
        }

        grid
    }
}