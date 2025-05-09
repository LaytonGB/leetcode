impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut res = vec![0; num1.len() + num2.len()];
        for (i,c) in num1.into_bytes().into_iter().enumerate().rev() {
            let mut carry = 0;
            for (j,d) in num2.as_bytes().into_iter().enumerate().rev() {
                let idx = i + j + 1;
                let next = (c - b'0') * (d - b'0') + res[idx] + carry;
                carry = next / 10;
                res[idx] = next % 10;
            }
            res[i] += carry;
        }
        if let Some(idx) = res.iter().position(|n| *n != 0) {
            res.into_iter().skip(idx).map(|n| (n + b'0') as char).collect()
        } else {
            "0".to_string()
        }
    }
}