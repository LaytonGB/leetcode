use std::collections::HashSet;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut res = HashSet::new();
        for k in 0..digits.len() {
            if digits[k] & 1 == 1 {
                continue;
            }
            
            for j in 0..digits.len() {
                if j == k {
                    continue;
                }

                for i in 0..digits.len() {
                    if i == j || i == k {
                        continue;
                    }

                    let x = digits[k] + digits[j] * 10 + digits[i] * 100;
                    if x >= 100 {
                        res.insert(x);
                    }
                }
            }
        }

        let mut res: Vec<i32> = res.into_iter().collect();
        res.sort_unstable();

        res
    }
}