impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let land_first_ans = Self::find_earliest_finish_after_time(
            Self::get_earliest_finish_time(&land_start_time, &land_duration),
            &water_start_time,
            &water_duration,
        );

        let water_first_ans = Self::find_earliest_finish_after_time(
            Self::get_earliest_finish_time(&water_start_time, &water_duration),
            &land_start_time,
            &land_duration,
        );

        land_first_ans.min(water_first_ans)
    }

    fn get_earliest_finish_time(starts: &[i32], durations: &[i32]) -> i32 {
        (0..starts.len()).map(|i| starts[i] + durations[i]).min().unwrap()
    }

    fn find_earliest_finish_after_time(time: i32, starts: &[i32], durations: &[i32]) -> i32 {
        (0..starts.len()).map(|i| time.max(starts[i]) + durations[i]).min().unwrap()
    }
}