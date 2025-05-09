use std::collections::VecDeque;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut s1 = VecDeque::with_capacity(students.len());
        s1.extend(students);
        let mut s2 = VecDeque::with_capacity(sandwiches.len());
        s2.extend(sandwiches);

        while !s2.is_empty() && count < s1.len() {
            if s1[0] != s2[0] {
                s1.rotate_left(1);
                count += 1;
            } else {
                s1.pop_front();
                s2.pop_front();
                count = 0;
            }
        }

        s1.len() as i32
    }
}