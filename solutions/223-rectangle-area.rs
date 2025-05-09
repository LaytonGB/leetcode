use std::cmp::{min, max};

impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1) - max(min(ax2,bx2) - max(ax1,bx1), 0) * max(min(ay2,by2) - max(ay1,by1), 0)
    }
}