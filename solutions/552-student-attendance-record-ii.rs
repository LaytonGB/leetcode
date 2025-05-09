const MOD: i64 = 1000000007;
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        ((1..n).fold([1, 1, 0, 1, 0, 0], |v, _| {
            [
                (v[0] + v[1] + v[2]) % MOD,
                v[0],
                v[1],
                (v[0] + v[1] + v[2] + v[3] + v[4] + v[5]) % MOD,
                v[3],
                v[4],
            ]
        }).into_iter().sum::<i64>() % MOD) as i32
    }
}