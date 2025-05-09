use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIRS: [usize; 5] = [0, 1, 0, !0, 0];

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());

        let mut q = BinaryHeap::from([(Reverse(0), (0, 0), 0)]);
        let mut v = vec![vec![false; m]; n];
        v[0][0] = true;
        while let Some((Reverse(time), (row, col), step)) = q.pop() {
            if row == n - 1 && col == m - 1 {
                return time;
            }

            for d in DIRS.windows(2) {
                let (r, c) = (row.wrapping_add(d[0]), col.wrapping_add(d[1]));
                if r < n && c < m && !v[r][c] {
                    v[r][c] = true;
                    q.push((
                        Reverse(move_time[r][c].max(time) + step + 1),
                        (r, c),
                        (step + 1) % 2)
                    );
                }
            }
        }

        unreachable!()
    }
}