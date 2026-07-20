use std::cmp::Ordering as O;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        (num1..=num2)
            .map(|mut x| {
                if x < 100 {
                    return 0;
                }
                
                let mut a = x % 10;
                x /= 10;

                let mut b = x % 10;
                x /= 10;

                let mut last_cmp = a.cmp(&b);

                let mut res = 0;
                while x > 0 {
                    let c = x % 10;
                    x /= 10;

                    let next_cmp = b.cmp(&c);
                    match (last_cmp, next_cmp) {
                        (O::Less, O::Greater) | (O::Greater, O::Less) => res += 1,
                        _ => {}
                    }

                    (a, b, last_cmp) = (b, c, next_cmp);
                }

                res
            })
            .sum()
    }
}