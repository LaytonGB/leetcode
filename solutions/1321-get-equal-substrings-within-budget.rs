use std::collections::VecDeque;
impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let mut cost = 0;
        let mut costs = VecDeque::new();
        let mut res = 0;
        for (a, b) in s.bytes().zip(t.bytes()) {
            let c = a.abs_diff(b) as i32;
            costs.push_back(c);
            cost += c;
            
            while cost > max_cost {
                cost -= costs.pop_front().unwrap();
            }

            res = res.max(costs.len());
        }

        res as i32
    }
}