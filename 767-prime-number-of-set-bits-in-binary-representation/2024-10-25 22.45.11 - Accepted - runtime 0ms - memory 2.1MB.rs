const PRIMES: [u32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        (left..=right)
            .filter(|i| PRIMES.binary_search(&i.count_ones()).is_ok())
            .count() as i32
    }
}