use std::collections::*;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let (mut days_7, mut days_30) = (VecDeque::<(i32, i32)>::new(), VecDeque::<(i32, i32)>::new());
        let mut cost = 0;
        for d in days {
            while !days_7.is_empty() && days_7.front().unwrap().0 + 7 <= d {
                days_7.pop_front();
            }
            while !days_30.is_empty() && days_30.front().unwrap().0 + 30 <= d {
                days_30.pop_front();
            }
            days_7.push_back((d, cost + costs[1]));
            days_30.push_back((d, cost + costs[2]));
            cost = (cost + costs[0]).min(days_7.front().unwrap().1).min(days_30.front().unwrap().1);
        }
        cost
    }
}