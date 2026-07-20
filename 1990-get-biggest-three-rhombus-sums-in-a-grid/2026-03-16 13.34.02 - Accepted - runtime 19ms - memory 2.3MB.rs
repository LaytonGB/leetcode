// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid/solutions/7651526/kotlin-rust-by-samoylenkodmitry-ol0f


const DIAGONALS: [usize; 5] = [1, 1, !0, !0, 1];

impl Solution {
    pub fn get_biggest_three(g: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (g.len(), g[0].len());
        let mut s = vec![];
        for y in 0..m {
            for x in 0..n {
                s.push(g[y][x]);

                let min_from_edge = x.min(n - x - 1).min(y).min(m - y - 1);
                for d in 1..=min_from_edge {
                    let (mut c, mut i, mut j) = (0, y-d, x);
                    for w in DIAGONALS.windows(2) {
                        let (dx, dy) = (w[0], w[1]);
                        for _ in 0..d {
                            c += g[i][j];
                            i = i.wrapping_add(dx);
                            j = j.wrapping_add(dy);
                        }
                    }
                    s.push(c);
                }
            }
        }
        
        s.sort_unstable_by_key(|&x| -x);
        s.dedup();
        s.truncate(3);
        s
    }
}