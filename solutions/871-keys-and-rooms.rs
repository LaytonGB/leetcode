use std::collections::HashSet;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut open = vec![false; rooms.len()];
        open[0] = true;
        let mut opened = 1;
        let mut stack = vec![0];
        while let Some(ri) = stack.pop() {
            for k in (&rooms[ri]).iter().map(|&k| k as usize) {
                if !open[k] {
                    opened += 1;
                    open[k] = true;
                    stack.push(k);
                }
            }
        }
        opened == rooms.len()
    }
}