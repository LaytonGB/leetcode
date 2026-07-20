// https://leetcode.com/problems/construct-product-matrix/solutions/4169695/pythonrust-flatten-prefix-suffix-bonus-b-q8p8
// Fantastic use of Iterator::scan

use std::iter::once;

const MOD: i32 = 12345;

impl Solution {
    pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let prefix = grid.iter()
            .flatten()
            // Accumulate values with prefix product
            .scan(1, |prev, curr| { *prev = (*prev * (*curr % MOD)) % MOD; Some(*prev) })
            .collect::<Vec<i32>>();

        let suffix = grid.iter()
            .flatten()
            .rev()
            .scan(1, |prev, curr| { *prev = (*prev * (*curr % MOD)) % MOD; Some(*prev) })
            .collect::<Vec<i32>>();

        once(&1)
            .chain(prefix.iter())
            .zip(suffix.iter().rev().skip(1).chain(once(&1)))
            .map(|(p, s)| (p * s) % 12345)
            .collect::<Vec<i32>>()
            .chunks(grid[0].len())
            .map(|c| c.to_vec())
            .collect::<Vec<Vec<i32>>>()
    }
}