impl Solution {
    pub fn solve_sudoku(mut board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(mut b: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if b[i][j] == '.' {
                    for n in "123456789".chars() {
                        if Self::is_valid(b, i, j, n) {
                            b[i][j] = n;
                            if Self::solve(b) {
                                return true;
                            }
                            b[i][j] = '.';
                        }
                    }

                    // got past 9 without recursing, must be impossible to fill this square
                    return false; 
                }
            }
        }

        // got to the end, grid must be full already
        true
    }

    fn is_valid(b: &Vec<Vec<char>>, i: usize, j: usize, n: char) -> bool {
        (0..9).all(|k| {
            if b[k][j] == n { return false; }
            if b[i][k] == n { return false; }
            if b[i / 3 * 3 + k / 3][j / 3 * 3 + k % 3] == n { return false; }
            true
        })
    }
}