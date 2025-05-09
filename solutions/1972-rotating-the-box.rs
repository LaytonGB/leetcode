impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (m, n) = (b.len(), b[0].len());
        let mut res = vec![vec!['.'; m]; n];
        for i in 0..m {
            for j in 0..n {
                res[j][m - i - 1] = b[i][j];
            }
        }
        println!("{:?}", res);

        for col in 0..m {
            let mut lowest_free = n - 1;
            for row in (0..n).rev() {
                match res[row][col] {
                    '.' => {}
                    '*' => {
                        lowest_free = row - 1;
                    }
                    '#' => {
                        res[row][col] = '.';
                        res[lowest_free][col] = '#';
                        lowest_free -= 1;
                    }
                    _ => unreachable!(),
                }
            }
        }

        res
    }
}