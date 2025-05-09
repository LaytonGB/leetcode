impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let n = customers.len();
        customers.into_iter()
            .fold((0_u64, 0_u64), |(wait, mut finish), c| {
                let start = finish.max(c[0] as u64);
                finish = start + c[1] as u64;
                (wait + finish - c[0] as u64, finish)
            })
            .0 as f64 / n as f64
    }
}