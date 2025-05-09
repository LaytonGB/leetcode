const C: [(usize, usize); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];

impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for (r, c) in C {
            let mut b_c = 0;
            let mut w_c = 0;
            
            for (i, j) in C {
                match grid[r + i][c + j] {
                    'B' => b_c += 1,
                    'W' => w_c += 1,
                    _ => unreachable!(),
                }
            }
            
            if b_c.max(w_c) >= 3 {
                return true;
            }
        }
        
        false
    }
}