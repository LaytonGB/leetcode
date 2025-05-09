impl Solution {
    pub fn max_task_assign(
        mut tasks: Vec<i32>,
        mut workers: Vec<i32>,
        pills: i32,
        strength: i32
    ) -> i32 {
        tasks.sort_unstable();
        workers.sort_unstable();
        Self::h(&tasks, &workers, pills, strength)
    }

    fn h(tasks: &[i32], workers: &[i32], pills: i32, strength: i32) -> i32 {
        let mut res = 0;
        let mut j = 0;
        for i in 0..tasks.len() {
            if j >= workers.len() {
                return res;
            }
            
            if tasks[i] <= workers[j] {
                res += 1;
            } else if pills > 0 && tasks[i] <= workers[j] + strength {
                return Self::h(&tasks[i..], &workers[i + 1..], pills, strength)
                    .max(Self::h(&tasks[i + 1..], &workers[i + 1..], pills - 1, strength) + 1)
                    + res;
            }

            j += 1;
        }

        res
    }
}