impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut cross_sum = 0;
        let mut prev_lasers = 0;
        for r in bank.iter() {
            let row_lasers = r.bytes().filter(|b| *b == b'1').count();
            if row_lasers > 0 {
                cross_sum += prev_lasers * row_lasers;
                prev_lasers = row_lasers
            }
        }
        return cross_sum as i32
    }
}