impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let mut size = 0;
                    let mut stack = Vec::from([(i,j)]);
                    while let Some((i,j)) = stack.pop() {
                        if grid[i][j] == 1 {
                            for (a,b) in [(i-1,j), (i+1,j), (i,j-1), (i,j+1)] {
                                if a < 0 || b < 0 || a >= grid.len() || b >= grid[0].len() { continue; }
                                if grid[a][b] == 1 {
                                    stack.push((a,b));
                                }
                            }
                            size += 1;
                            grid[i][j] = 0;
                        }
                    }
                    if size > res {
                        res = size;
                    }
                }
            }
        }
        res
    }
}