use std::iter::once;

impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (lower, upper) = (lower as i64, upper as i64);
        
        let (mut min, mut max) = (0_i64, 0_i64);
        let mut last = 0_i64;
        for d in differences {
            let next = last.checked_add(d as i64).expect("out of bounds");
            min = min.min(next);
            max = max.max(next);
            last = next;
        }

        let diff = lower - min;
        min += diff;
        max += diff;

        if upper - max >= 0 {
            (upper - max + 1) as i32
        } else {
            0
        }
    }
}