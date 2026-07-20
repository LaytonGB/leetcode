impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        (0..12_u8)
            .map(|h| (h, h.count_ones()))
            .flat_map(|(h, ho)| (0..60_u8)
                .map(|m| (m, m.count_ones()))
                .map(move |(m, mo)| ((h, ho), (m, mo)))
            )
            .filter(|((h, ho), (m, mo))| ho + mo == turned_on)
            .map(|((h, _), (m, _))| format!("{}:{:02}", h, m))
            .collect()
    }
}