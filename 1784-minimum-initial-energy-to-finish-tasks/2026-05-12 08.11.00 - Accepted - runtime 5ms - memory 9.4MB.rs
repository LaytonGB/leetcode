use std::cmp::Reverse;

impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.sort_unstable_by_key(|t| t[1] - t[0]);

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