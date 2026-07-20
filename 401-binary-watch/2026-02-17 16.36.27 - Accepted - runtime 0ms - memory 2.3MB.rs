impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut res = vec![];
        for h in 0..12_u8 {
            let h_ones = h.count_ones();
            for m in 0..60_u8 {
                if h_ones + m.count_ones() == turned_on {
                    res.push(format!("{}:{:02}", h, m));
                }
            }
        }
        res
    }
}