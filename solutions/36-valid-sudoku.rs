impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 9 boxes, 9 rows, 9 columns - create 3 arrays
        let mut rows: [u16; 9] = [0; 9];
        let mut cols: [u16; 9] = [0; 9];
        let mut boxes: [u16; 9] = [0; 9];
        
        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] // for each grid square
                {
                    '.' => continue, // ignore dots
                    c => { // process string number
                        let b: usize = (i / 3) * 3 + (j / 3); // get relevant square?
                        let curr = 1 << (c.to_digit(10).unwrap()); // convert c to number, bit shift 1 by that amount (max 9)
                        
                        // catch any duplicate occurrences that would make this sodoku invalid
                        if rows[i] & curr != 0 || cols[j] & curr != 0 || boxes[b] & curr != 0 {
                            return false;
                        }
    
                        // record new value
                        rows[i] |= curr;
                        cols[j] |= curr;
                        boxes[b] |= curr;
                    }
                }
            }
        }
        
        true
    }
}