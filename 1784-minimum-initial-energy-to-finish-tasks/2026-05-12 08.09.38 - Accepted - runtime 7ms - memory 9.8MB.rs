use std::cmp::Reverse;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let tasks = {
            let mut tasks: Vec<[i32; 2]> = tasks.into_iter().map(|t| [t[0], t[1]]).collect();
            tasks.sort_unstable_by_key(|t| t[1] - t[0]);
            tasks
        };

        tasks.into_iter().rev()
            .fold([0, 0], |[curr, total], t| {
                let added = if curr < t[1] {
                    t[1] - curr
                } else {
                    0
                };

                [curr + added - t[0], total + added]
            })[1]
    }
}