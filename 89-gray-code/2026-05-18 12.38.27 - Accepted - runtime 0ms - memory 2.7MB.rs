impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![0, 1];
        
        if n == 1 {
            return res;
        }

        for i in 1..n {
            res.append(
                &mut res.iter()
                    .rev()
                    .map(|x| x + 2_i32.pow(i.cast_unsigned()))
                    .collect()
            );
        }

        res
    }
}