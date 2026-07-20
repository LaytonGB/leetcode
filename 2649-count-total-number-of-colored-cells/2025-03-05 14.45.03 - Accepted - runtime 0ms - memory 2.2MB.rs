impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        // 2n^2 - 2n + 1
        2 * (n as i64).pow(2) - 2 * n as i64 + 1
    }
}