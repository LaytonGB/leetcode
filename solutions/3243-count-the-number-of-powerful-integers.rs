fn all_digits_at_most_limit(mut x: i64, limit: i64) -> bool {
    while x > 0 {
        if x % 10 > limit {
            return false;
        }

        x /= 10;
    }
    true
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let s_size = 10_i64.pow(s.len() as u32);
        if s_size > finish {
            return 0;
        }

        let s_num = s.parse::<i64>().unwrap();
        let limit_s = limit.to_string();

        let start_rem = start % s_size;
        let start = if start_rem > s_num {
            start + s_size
        } else {
            start
        };

        let finish_rem = finish % s_size;
        let finish = if finish_rem < s_num {
            finish - s_size
        } else {
            finish
        };

        let mut res = 0;
        for i in start / s_size..=finish/s_size {
            if all_digits_at_most_limit(i, limit as i64) {
                res += 1;
            }
        }

        println!(
            "s_num:{}, s_size:{}, start_rem:{}, start:{}, finish_rem:{}, finish:{}",
            s_num,
            s_size,
            start_rem,
            start,
            finish_rem,
            finish
        );

        res
    }
}