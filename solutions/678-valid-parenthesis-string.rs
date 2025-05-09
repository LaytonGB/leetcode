impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min = 0;
        let mut max = 0;

        for (i, c) in s.chars().enumerate() {
            match c {
                '*' => {
                    min = 0.max(min - 1);
                    max += 1;
                }
                '(' => {
                    min += 1;
                    max += 1;
                }
                ')' => {
                    if max == 0 {
                        return false;
                    }

                    min = 0.max(min - 1);
                    max -= 1;
                }
                _ => unreachable!(),
            }
        }

        min == 0
    }
}