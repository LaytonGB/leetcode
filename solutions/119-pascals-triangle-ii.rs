use std::collections::HashMap;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return Vec::from([1]);
        } else if row_index == 1 {
            return Vec::from([1, 1]);
        }
        
        let last = Self::get_row(row_index-1);
        let mut out = Vec::<i32>::with_capacity((row_index+1) as usize);
        out.push(last[0]);
        for i in 1..last.len() {
            out.push(last[i-1] + last[i]);
        }
        out.push(last[last.len()-1]);
        out
    }
}