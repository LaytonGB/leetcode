impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let k = k as usize;
        
        // get free times
        let mut free_times = vec![0; n + 1];
        for i in 0..=n {
            let prev_end = *end_time.get(i - 1).unwrap_or(&0);
            let this_start = *start_time.get(i).unwrap_or(&event_time);

            free_times[i] = this_start - prev_end;
        }

        // window across k free times, keep largest
        let mut free_size_sum = free_times.iter().take(k + 1).sum::<i32>();
        let mut free_size_max = free_size_sum;
        for i in 0..n - k {
            free_size_sum -= free_times[i];
            free_size_sum += free_times[i + k + 1];

            free_size_max = free_size_max.max(free_size_sum);
        }

        free_size_max
    }
}