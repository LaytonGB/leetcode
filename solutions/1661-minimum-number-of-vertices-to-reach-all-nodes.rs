use std::collections::*;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut incomings = vec![0; n];
        for e in edges {
            incomings[e[1] as usize] += 1;
        }
        incomings
            .into_iter()
            .enumerate()
            .filter_map(|(i,x)| if x == 0 {Some(i as i32)} else {None})
            .collect()
    }
}