impl Solution {
    pub fn compare_version(v1: String, v2: String) -> i32 {
        let mut i1 = v1.chars().enumerate();
        let mut i2 = v2.chars().enumerate();

        let mut i = 0;
        let mut j = 0;

        loop {
            let (x, new_i) = Self::next_version(&mut i1, &v1, i);
            let (y, new_j) = Self::next_version(&mut i2, &v2, j);

            i = new_i;
            j = new_j;

            match (x, y) {
                (Some(x), Some(y)) => if x < y {
                    return -1;
                } else if x > y {
                    return 1;
                },
                (Some(x), None) => if x > 0 {
                    return 1;
                },
                (None, Some(y)) => if y > 0 {
                    return -1;
                }
                (None, None) => return 0,
            }
        }
    }

    fn next_version(it: &mut impl Iterator<Item = (usize, char)>, v: &str, i: usize) -> (Option<i32>, usize) {
        for (j, c) in it {
            match c {
                '.' => return (v[i..j].parse::<i32>().ok(), j + 1),
                _ => (),
            }
        }
        (v[i..].parse::<i32>().ok(), v.len())
    }
}