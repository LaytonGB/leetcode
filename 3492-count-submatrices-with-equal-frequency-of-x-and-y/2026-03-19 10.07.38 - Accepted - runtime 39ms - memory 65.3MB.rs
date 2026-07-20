use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Count {
    x: i32,
    y: i32,
}

impl Count {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn is_any_empty(&self) -> bool {
        self.x == 0 || self.y == 0
    }

    pub fn is_equal(&self) -> bool {
        self.x == self.y
    }
}

impl From<char> for Count {
    fn from(value: char) -> Self {
        if value == 'X' {
            Self {
                x: 1,
                y: 0,
            }
        } else if value == 'Y' {
            Self {
                x: 0,
                y: 1,
            }
        } else {
            Self::new()
        }
    }
}

impl Add<Self> for Count {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Self> for Count {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let (h, w) = (grid.len(), grid[0].len());
        let mut row_sum = vec![vec![Count::new(); w]; h];
        let mut col_sum = vec![vec![Count::new(); w]; h];

        let mut res = 0;
        for y in 0..h {
            for x in 0..w {
                if x > 0 {
                    row_sum[y][x] = Count::from(grid[y][x]) + row_sum[y][x-1];
                } else {
                    row_sum[y][x] = Count::from(grid[y][x]);
                }

                if y > 0 {
                    col_sum[y][x] = row_sum[y][x] + col_sum[y-1][x];
                } else {
                    col_sum[y][x] = row_sum[y][x];
                }

                if !col_sum[y][x].is_any_empty() && col_sum[y][x].is_equal() {
                    // println!("keeping y:{} x:{} val:{:?}", y, x, col_sum[y][x]);
                    res += 1;
                }
            }
        }

        // println!("end row_sum: {:?}", row_sum);
        // println!("end col_sum: {:?}", col_sum);

        res
    }
}