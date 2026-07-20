const HOUR: f64 = 360. / 12.;
const HOUR_ADJ: f64 = HOUR / 60.;
const MIN: f64 = 360. / 60.;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let h = HOUR * (hour % 12) as f64 + HOUR_ADJ * minutes as f64;
        let m = MIN * minutes as f64;
        let angle = (h - m).abs();
        if angle > 180. {
            360. - angle
        } else {
            angle
        }
    }
}