impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut out: u32 = 0;
        let mut i: u32 = 0;
        for c in column_title.chars().rev().into_iter() {
            out += (c as u32 - 64) * 26_u32.pow(i);
            i += 1;
        }
        out as i32
    }
}