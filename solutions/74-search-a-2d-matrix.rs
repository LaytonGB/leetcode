impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (y,x) = (matrix.len(), matrix[0].len());
        let row; let mut l; let mut h; let mut z1; let mut z2; let mut m = 0;
        if y > 1 {
            // binary search rows
            l = 0; h = y;
            while l < h {
                m = l + ((h - l) / 2);
                z1 = &matrix[m][0];
                z2 = &matrix[m][x-1];
                if z2 < &target {
                    l = m + 1;
                } else if z1 > &target {
                    h = m;
                } else {
                    break;
                }
            }
            row = &matrix[m];
        } else {
            row = &matrix[0];
        }
        println!("{row:?}");
        // binary search for target on row
        l = 0; h = x;
        while l < h {
            m = l + ((h - l) / 2);
            z1 = &row[m];
            println!("{z1:?}");
            if z1 == &target {
                return true;
            } else if z1 < &target {
                l = m + 1;
            } else {
                h = m;
            }
        }
        false
    }
}