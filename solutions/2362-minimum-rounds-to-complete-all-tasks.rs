use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut task_counts = tasks.into_iter().fold(HashMap::new(), |mut m, t| { *m.entry(t).or_insert(0) += 1; m });
        let mut res = 0;
        for (t, mut n) in task_counts.into_iter() {
            if n == 1 { return -1; }
            else { res += (n + 2) / 3; }
        }
        res
    }
}