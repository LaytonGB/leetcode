impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nu = turned_on as u32;
        let mut out = Vec::new();
        for i in 0u32..12 {
            for j in 0u32..60 {
                if i.count_ones() + j.count_ones() == nu {
                    out.push(format!("{}:{:02}", i, j));
                }
            }
        }
        out
    }
}