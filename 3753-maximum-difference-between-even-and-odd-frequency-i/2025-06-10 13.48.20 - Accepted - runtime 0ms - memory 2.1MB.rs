use std::collections::HashMap;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut map = [0; 26];
        for b in s.bytes() {
            map[(b - b'a') as usize] += 1;
        }
        map.sort_unstable();

        let first_non_zero_index = map.partition_point(|x| *x == 0);
        let values = &map[first_non_zero_index..];

        let o = values.iter().rev().filter(|x| *x & 1 == 1).next().unwrap();
        let e = values.iter().filter(|x| *x & 1 == 0).next().unwrap();

        o - e
    }
}