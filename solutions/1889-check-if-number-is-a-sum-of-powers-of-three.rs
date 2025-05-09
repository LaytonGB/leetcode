use std::cmp::Ordering as O;

const fn get_powers_of_3() -> [i32; 15] {
    let mut res = [1; 15];
    res[15 - 2] = 3;

    let mut i = 12;
    let mut n = 3;
    let lim = 10_i32.pow(7);
    while n * 3 < lim {
        n *= 3;
        res[i] = n;
        i -= 1;
    }

    res
}

const POWERS_OF_3: [i32; 15] = get_powers_of_3();

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        Self::inner(n, 0)
    }

    fn inner(n: i32, start: usize) -> bool {
        for (i, m) in POWERS_OF_3.iter().enumerate().skip(start) {
            match n.cmp(m) {
                O::Less => {}
                O::Equal => {
                    return true;
                }
                O::Greater => {
                    if Self::inner(n - *m, i + 1) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
