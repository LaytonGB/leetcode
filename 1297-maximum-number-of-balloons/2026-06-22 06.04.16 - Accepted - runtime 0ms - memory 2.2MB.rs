impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts = [0; 5];
        for c in text.chars() {
            match c {
                'b' => counts[0] += 1,
                'a' => counts[1] += 1,
                'l' => counts[2] += 1,
                'o' => counts[3] += 1,
                'n' => counts[4] += 1,
                _ => {}
            }
        }
        counts[2] /= 2;
        counts[3] /= 2;

        counts.into_iter().min().unwrap()
    }
}