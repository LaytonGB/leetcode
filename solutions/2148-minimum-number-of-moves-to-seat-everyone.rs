impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();
        seats.into_iter()
            .zip(students.into_iter())
            .fold(0, |res, (seat, student)| res + seat.abs_diff(student)) as i32
    }
}