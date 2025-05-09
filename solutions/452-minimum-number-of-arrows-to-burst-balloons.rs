use std::cmp::Ordering as O;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x| x[0]);
        points.iter()
            .skip(1)
            .fold((1, points[0][1]), |(res, end), x| {
                if x[0] > end {
                    (res + 1, x[1])
                } else {
                    (res, end.min(x[1]))
                }
            })
            .0
    }
}