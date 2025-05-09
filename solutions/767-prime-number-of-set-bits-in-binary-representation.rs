const PRIMES: [bool; 21] = [false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false];

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right)
            .filter(|i| PRIMES[i.count_ones() as usize])
            .count() as i32
    }
}