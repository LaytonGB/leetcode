use std::collections::VecDeque;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut k = k as usize;
        let mut tickets = VecDeque::from(tickets);
        let mut res = 0;
        loop {
            let Some(t) = tickets.pop_front() else {
                unreachable!()
            };

            if k == 0 {
                if t == 1 {
                    return res + 1;
                }
                k = tickets.len();
            } else {
                k = k - 1;
            };
            
            if t != 1 {
                tickets.push_back(t - 1);
            }

            res += 1;
        }
        unreachable!()
    }
}