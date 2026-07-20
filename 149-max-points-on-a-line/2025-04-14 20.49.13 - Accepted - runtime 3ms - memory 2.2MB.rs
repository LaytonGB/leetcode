use std::collections::HashMap;

// https://docs.rs/num-integer/0.1.41/src/num_integer/lib.rs.html#459
fn get_gcd(mut m: i32, mut n: i32) -> i32 {
    if m == 0 || n == 0 {
        return (m | n).abs();
    }

    let shift = (m | n).trailing_zeros();

    if m == i32::MIN || n == i32::MIN {
        return (1_i32 << shift).abs();
    }

    m = m.abs();
    n = n.abs();

    m >>= m.trailing_zeros();
    n >>= n.trailing_zeros();

    while m != n {
        if m > n {
            m -= n;
            m >>= m.trailing_zeros();
        } else {
            n -= m;
            n >>= n.trailing_zeros();
        }
    }
    m << shift
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 1 {
            return 1;
        }

        let mut res = 0;
        for i in 0..n - 1 {
            let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
            let mut duplicates_count = 1;
            for j in i + 1..n {
                let (p1, p2) = (&points[i], &points[j]);
                if p1 == p2 {
                    duplicates_count += 1;
                    continue;
                }

                let (mut dy, mut dx) = (p1[1] - p2[1], p1[0] - p2[0]);
                (dy, dx) = match (dy, dx) {
                    (..=-1, _) | (0, ..=-1) => (dy * -1, dx * -1),
                    _ => (dy, dx),
                };
                let gcd = get_gcd(dy, dx);
                let key = (dy / gcd, dx / gcd);
                *counts.entry(key).or_default() += 1;
            }

            res = res.max(*counts.values().max().unwrap() + duplicates_count);
        }

        res
    }
}
