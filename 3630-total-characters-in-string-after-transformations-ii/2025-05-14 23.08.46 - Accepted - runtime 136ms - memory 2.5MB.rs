const MOD: usize = 10_usize.pow(9) + 7;
const N: usize = 26;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut transform = vec![vec![0; N]; N];
        for i in 0..N {
            for shift in 0..nums[i] as usize {
                transform[i][(i + 1 + shift) % N] += 1;
            }
        }
        transform = Self::mat_pow(transform, t as usize);

        let mut freq = vec![vec![0; N]];
        s.bytes().for_each(|b| freq[0][(b - b'a') as usize] += 1);
        freq = Self::mat_mul(&freq, &transform);

        (freq[0].iter().fold(0, |acc, f| (acc + f) % MOD)) as i32
    }

    fn mat_pow(mut m: Vec<Vec<usize>>, mut e: usize) -> Vec<Vec<usize>> {
        let mut res = vec![vec![0; N]; N];
        (0..N).for_each(|i| res[i][i] = 1 );
        while e > 0 {
            if e & 1 == 1 {
                res = Self::mat_mul(&res, &m);
            }
            m = Self::mat_mul(&m, &m);
            e /= 2;
        }
        res
    }

    fn mat_mul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let (a_rows, a_cols, b_cols) = (a.len(), a[0].len(), b[0].len());
        let mut res = vec![vec![0; b_cols]; a_rows];
        for i in 0..a_rows {
            for j in 0..b_cols {
                res[i][j] = (0..a_cols).fold(0, |acc, k| (acc + a[i][k] * b[k][j]) % MOD);
            }
        }
        res
    }
}