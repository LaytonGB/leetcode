const MOD: i64 = 10_i64.pow(9) + 7;
use std::collections::*;
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None; fuel as usize + 1]; locations.len()]; // dp[position][fuel]
        Self::dfs(&locations, &mut dp, finish as usize, start as usize, fuel) as i32
    }

    fn dfs(
        locations: &Vec<i32>,
        dp: &mut Vec<Vec<Option<i64>>>,
        finish: usize,
        pos: usize,
        fuel: i32
    ) -> i64 {
        let mut res = 0_i64;
        if let Some(r) = dp[pos][fuel as usize] {
            res += r;
        } else {
            if pos == finish {
                res = (res + 1) % MOD;
            }
            if fuel > 0 {
                for i in 0..locations.len() {
                    let new_fuel = fuel - (locations[pos] - locations[i]).abs();
                    if i != pos && new_fuel >= 0 {
                        res = (res + Self::dfs(locations, dp, finish, i, new_fuel)) % MOD;
                    }
                }
            }
            dp[pos][fuel as usize] = Some(res);
        }
        return res;
    }
}