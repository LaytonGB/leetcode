use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn least_interval(mut tasks: Vec<char>, n: i32) -> i32 {
        // make vec of task counts, eg ['A', 'A', 'B'] -> [2, 1]
        let mut task_counts = tasks
            .iter()
            .fold(vec![0_i32; 26], |mut v, x| {
                v[((*x) as u8 - b'A') as usize] += 1;
                v
            });
        task_counts.sort_unstable();

        // calculate lowest frequency, ideally get a negative
        let max_freq = task_counts[25] - 1;
        let mut idle_time = max_freq * n;

        if idle_time < 0 {
            tasks.len() as i32
        } else {
            for i in (0..=24).rev() {
                idle_time -= max_freq.min(task_counts[i]);
            }
            if idle_time < 0 {
                tasks.len() as i32
            } else {
                tasks.len() as i32 + idle_time
            }
        }
    }
}