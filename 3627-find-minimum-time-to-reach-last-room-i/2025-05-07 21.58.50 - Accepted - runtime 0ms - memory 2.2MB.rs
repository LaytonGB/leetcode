use std::collections::BinaryHeap;
use std::cmp::Reverse;

const DIR: [usize; 5] = [0, 1, 0, !0, 0];

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (move_time.len(), move_time[0].len());
        let mut q = BinaryHeap::from([(Reverse(0), (0, 0))]);
        let mut v = vec![vec![false; m]; n];
        while let Some((Reverse(time), (row, col))) = q.pop() {
            if row == n - 1 && col == m - 1 {
                return time;
            }

            for d in DIR.windows(2) {
                let (r, c) = (row + d[0], col + d[1]);

                if r < n && c < m && !v[r][c] {
                    q.push((Reverse(time.max(move_time[r][c]) + 1), (r, c)));
                    v[r][c] = true;
                }
            }
        }

        unreachable!()
    }
}