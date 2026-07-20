use std::collections::VecDeque;

const MOD: usize = 10_usize.pow(9) + 7;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut counts = VecDeque::from([0; 26]);
        s.bytes()
            .map(|b| (b - b'a') as usize)
            .for_each(|i| counts[i] += 1);

        for _ in 0..t {
            if counts[25] > 0 {
                counts[0] += counts[25] % MOD;
            }
            counts.rotate_right(1);
        }

        (counts.into_iter().sum::<usize>() % MOD) as i32
    }
}