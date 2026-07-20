impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.into_iter()
            .filter(|d| {
                let mut chars = d.chars().skip(11);
                let decades = chars.next().unwrap();
                let years = chars.next().unwrap();
                matches!((decades, years), ('7'.., _) | ('6', '1'..))
            })
            .count() as i32
    }
}