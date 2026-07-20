use std::cmp::Ordering as O;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut res = 0;
        for x in num1..=num2 {
            res += Self::get_waviness(x);
        }
        res
    }

    fn get_waviness(mut x: i32) -> i32 {
        if x < 100 {
            return 0;
        }

        let mut digits = vec![];
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }

        let mut res = 0;
        for w in digits.windows(3) {
            match (w[1].cmp(&w[0]), w[1].cmp(&w[2])) {
                (O::Less, O::Less) | (O::Greater, O::Greater) => res += 1,
                _ => {}
            }
        }

        res
    }
}