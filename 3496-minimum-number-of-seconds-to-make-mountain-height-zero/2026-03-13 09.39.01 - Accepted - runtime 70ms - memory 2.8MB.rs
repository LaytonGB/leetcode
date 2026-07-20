use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialOrd, Ord)]
struct Worker {
    potential_cost: i64,
    next_cost: i64,
    original_cost: i64,
    total_cost: i64,
    idx: usize,
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.potential_cost == other.potential_cost
        && self.next_cost == other.next_cost
        && self.original_cost == other.original_cost
        && self.total_cost == other.total_cost
    }
}

impl Worker {
    fn new(cost: i32, idx: usize) -> Self {
        Worker {
            potential_cost: cost as i64,
            next_cost: cost as i64,
            original_cost: cost as i64,
            total_cost: 0,
            idx,
        }
    }

    fn work(&mut self) {
        self.total_cost += self.next_cost;
        self.next_cost += self.original_cost;
        self.potential_cost += self.next_cost;
    }
}

impl Solution {
    pub fn min_number_of_seconds(mut mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let n = worker_times.len();
        let mut workers = worker_times.into_iter()
            .enumerate()
            .fold(BinaryHeap::with_capacity(n), |mut res, (i, w)| {
                res.push(Reverse(Worker::new(w, i)));
                res
            });

        while mountain_height > 0 {
            let Some(Reverse(mut w)) = workers.pop() else { break };
            // println!("worker {}", w.idx);
            w.work();
            workers.push(Reverse(w));
            mountain_height -= 1;
        }

        let Some(Reverse(mut last_worker)) = workers.pop() else { return -1 };
        while let Some(Reverse(w)) = workers.pop() {
            if last_worker.total_cost < w.total_cost {
                last_worker = w;
            }
        }

        last_worker.total_cost
    }
}