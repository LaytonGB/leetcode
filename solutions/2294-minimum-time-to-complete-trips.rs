impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let (mut l, mut h) = (0_i64, total_trips * *time.iter().min().unwrap() as i64);
        let mut m;
        while l < h {
            m = l + ((h - l) / 2);
            let trips = time.iter().fold(0_i64, |a,b| a + (m / *b as i64));
            if trips < total_trips {
                l = m + 1;
            } else {
                h = m;
            }
        }
        l
    }
}