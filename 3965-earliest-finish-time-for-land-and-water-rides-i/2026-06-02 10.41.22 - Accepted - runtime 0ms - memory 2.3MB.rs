impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let (m, n) = (land_start_time.len(), water_start_time.len());
        
        let land_earliest_finish = Self::get_earliest_finish(&land_start_time, &land_duration);
        let water_earliest_finish = Self::get_earliest_finish(&water_start_time, &water_duration);

        Self::find_min_finish(land_earliest_finish, &water_start_time, &water_duration)
            .min(Self::find_min_finish(water_earliest_finish, &land_start_time, &land_duration))
    }

    fn find_min_finish(prev_fin: i32, starts: &[i32], durations: &[i32]) -> i32 {
        (0..starts.len()).map(|i| prev_fin.max(starts[i]) + durations[i]).min().unwrap()
    }

    fn get_earliest_finish(starts: &[i32], durations: &[i32]) -> i32 {
        let mut a_fin: Vec<i32> = (0..starts.len()).map(|i| starts[i] + durations[i]).collect();
        a_fin.sort_unstable();
        a_fin[0]
    }
}