fn sum_digits(n: i32) -> i32 {
    n.to_string()
        .bytes()
        .map(|b| (b - b'0') as i32)
        .sum::<i32>()
}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut res = 0;
        for i in low..=high {
            let len = i.ilog10() + 1;
            if len & 1 == 1 {
                continue;
            }

            let n = len / 2;

            let a = i / 10_i32.pow(n);
            let b = i % 10_i32.pow(n);
            // println!("i:{} a:{} b:{}", i, a, b);

            if a > 0 && b > 0 && sum_digits(a) == sum_digits(b) {
                // println!("adding");
                res += 1;
            }
        }

        res
    }
}