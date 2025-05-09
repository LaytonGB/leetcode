use std::collections::{BinaryHeap, VecDeque};

const D: [usize; 5] = [0, 1, 0, !0, 0];

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return 0;
        }

        let scores = Self::get_scores(&grid);

        let mut v = vec![vec![false; n]; n];
        let mut pq = BinaryHeap::with_capacity(n * n);
        pq.push((scores[0][0], (0, 0)));
        loop {
            let (d, (i, j)) = pq.pop().unwrap();
            if v[i][j] {
                continue;
            }
            if i == n - 1 && j == n - 1 {
                return d;
            }

            v[i][j] = true;

            for w in D.windows(2) {
                let (i, j) = (i + w[0], j + w[1]);
                if i < n && j < n {
                    pq.push((d.min(scores[i][j]), (i, j)));
                }
            }
        }

        unreachable!()
    }

    fn get_scores(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        let mut res = vec![vec![i32::MAX; n]; n];
        for (i, j) in (0..n).flat_map(|i| (0..n).map(move |j| (i, j))) {
            if grid[i][j] == 1 {
                Self::update_scores_from(&mut res, i, j);
            }
        }

        res
    }

    fn update_scores_from(scores: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        let n = scores.len();
        
        let mut q = VecDeque::with_capacity(n * n);
        q.push_back((i, j));
        let mut d = 0;
        while !q.is_empty() {
            let l = q.len();
            for _ in 0..l {
                let (i, j) = q.pop_front().unwrap();
                if scores[i][j] <= d {
                    continue;
                }

                scores[i][j] = d;

                for w in D.windows(2) {
                    let (i, j) = (i + w[0], j + w[1]);
                    if i < n && j < n {
                        q.push_back((i, j));
                    }
                }
            }
            d += 1;
        }
    }
}