use std::cmp::Ordering as O;

const W: [i64; 570] = get_w();

const fn get_w() -> [i64; 570] {
    let mut waves = [0; 570];
    let mut last = 0;
    let mut i = 0;
    while i < 1000 {
        let r = i % 10;
        let m = (i / 10) % 10;
        let l = (i / 100) % 10;
        let max = if l > r { l } else { r };
        let min = if l < r { l } else { r };
        if (m > max) | (m < min) {
            waves[last] = i;
            last += 1;
        }
        i += 1;
    }
    waves
}

impl Solution {
    pub fn total_waviness(a: i64, b: i64) -> i64 {
        Self::wave_count(b) - Self::wave_count(a - 1)
    }

    fn wave_count(x: i64) -> i64 {
        if x < 100 {
            return 0;
        }

        W.iter()
            .map(|&p| Self::count_ways(x, p))
            .sum()
    }

    fn count_ways(x: i64, p: i64) -> i64 {
        let s = x.to_string();
        let n = s.len();
        let t = (p < 100) as i64;
        let mut count = 0;
        for i in 0..n - 2 {
            let pre = (&s[..i]).parse::<i64>().unwrap_or(0);
            let cur = (&s[i..i+3]).parse::<i64>().unwrap();
            let suf = (&s[i+3..]).parse::<i64>().unwrap_or(0);
            let mult = 10_i64.pow((n - i - 3) as u32) as i64;
            let mut ways = 0;
            let mut edge = 0;

            match cur.cmp(&p) {
                O::Greater => ways = pre - t + 1,
                O::Equal => {
                    ways = 0.max(pre - t);
                    edge = (pre >= t) as i64 * (suf + 1);
                }
                O::Less => ways = 0.max(pre - t),
            }

            count += ways * mult + edge;
        }
        count
    }
}