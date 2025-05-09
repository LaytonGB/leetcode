use std::collections::*;
use std::cmp::Reverse;
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        if events.len() == 0 {
            return 0;
        }
        let mut end_days: HashMap<i32, Vec<Reverse<(i32, i32)>>> = events.iter()
            .fold(HashMap::new(), |mut m, e| {
                m.entry(e[0])
                    .and_modify(|d| d.push(Reverse((e[1], e[0]))))
                    .or_insert(vec![Reverse((e[1], e[0]))]);
                m
            });
        let mut ongoing = BinaryHeap::new();
        let (mut i, last_day) = events.iter().fold((i32::MAX, i32::MIN), |(l, h), e| {
            (l.min(e[0]), h.max(e[1]))
        });
        while !ongoing.is_empty() || i <= last_day {
            // println!("i:{}\n---\n{:?}\n{:?}\n---", i, end_days, ongoing);
            if let Some(mut ending_days) = end_days.remove(&i) {
                ongoing.extend(&mut ending_days.into_iter());
            }
            while let Some(Reverse(end_day)) = ongoing.pop() {
                // println!("{:?}\n===", end_day);
                if end_day.0 >= i {
                    res += 1;
                    break;
                }
            }
            i += 1;
        }
        res
    }
}