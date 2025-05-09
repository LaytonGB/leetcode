impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|a| a[1]);

        let mut accepted_count = 1;
        let mut last_end = intervals[0][1];
        for interval in intervals.iter().skip(1) {
            let (start, end) = (interval[0], interval[1]);
            if last_end <= start {
                last_end = end;
                accepted_count += 1;
            }
        }

        intervals.len() as i32 - accepted_count
    }
}