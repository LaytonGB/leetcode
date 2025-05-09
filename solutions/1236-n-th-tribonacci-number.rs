use std::collections::VecDeque;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut q = VecDeque::from([0, 1, 1]);
        match n {
            0..=2 => q[n as usize],
            _ => {
                for _ in 2..n {
                    let x = q[0] + q[1] + q[2];
                    q.pop_front();
                    q.push_back(x);
                }
                q[2]
            }
        }
    }
}