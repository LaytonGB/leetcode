use std::collections::VecDeque;

const DIRS: [usize; 4] = [1, 10, 100, 1000];
const UNKNOWN: i32 = 10_000;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut visited = [UNKNOWN; 10_000];
        let target = target.parse::<usize>().unwrap();
        for d in deadends {
            let d = d.parse::<usize>().unwrap();
            if d == 0 {
                return -1;
            }
            visited[d] = -1;
        }
        visited[0] = 0;

        let mut q = VecDeque::new();
        q.push_back(0);
        while let Some(n) = q.pop_front() {
            for d in DIRS {
                let num = (n / d) % 10;
                let dist = visited[n];
                match num {
                    0 => {
                        Self::add(&mut visited, &mut q, n + d, dist);
                        Self::add(&mut visited, &mut q, n + 9 * d, dist);
                    }
                    9 => {
                        Self::add(&mut visited, &mut q, n - 9 * d, dist);
                        Self::add(&mut visited, &mut q, n - d, dist);
                    }
                    _ => {
                        Self::add(&mut visited, &mut q, n + d, dist);
                        Self::add(&mut visited, &mut q, n - d, dist);
                    }
                }
            }

            if visited[target] != UNKNOWN {
                return visited[target];
            }
        }

        -1
    }

    fn add(visited: &mut [i32; 10000], q: &mut VecDeque<usize>, neighbor: usize, dist: i32) {
        if visited[neighbor] == UNKNOWN {
            q.push_back(neighbor);
            visited[neighbor] = dist + 1;
        }
    }
}