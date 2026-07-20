use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

impl Solution {
    pub fn count_days(days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        meetings.into_iter().for_each(|meeting| {
            let [start, end] = &meeting[..] else { unreachable!() };
            
            m.entry(Reverse(*start))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            
            m.entry(Reverse(end + 1))
                .and_modify(|e| *e -= 1)
                .or_insert(-1);
        });

        let mut c = BinaryHeap::from_iter([(Reverse(1), 0)].into_iter().chain(m.into_iter()));
        let mut last_zero_day = 1;
        let mut meetings = 0;
        let mut res = 0;
        while let  Some((Reverse(day), daily_meetings)) = c.pop() {
            let next_meetings = meetings + daily_meetings;
            match (meetings, next_meetings) {
                (0, 1..) => res += day - last_zero_day,
                (1.., 0) => last_zero_day = day,
                _ => {}
            }

            meetings = next_meetings;
        }

        res - last_zero_day + days + 1
    }
}