fn is_prime(n: i32) -> bool {
    match n {
        ..=1 => false,
        n => (2..n / 2 + 1).all(|m| n % m != 0),
    }
}

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = vec![];

        for i in left..=right {
            if is_prime(i) {
                primes.push(i);
            }
        }

        primes.windows(2)
            .reduce(|w1, w2| {
                if w1[1] - w1[0] <= w2[1] - w2[0] {
                    w1
                } else {
                    w2
                }
            })
            .unwrap_or(&vec![-1, -1])
            .to_vec()
    }
}