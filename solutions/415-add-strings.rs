impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut carry = 0;
        let mut out = Vec::new();
        let (mut n1, mut n2) = if num1.len() >= num2.len() {(num1,num2)} else {(num2,num1)};
        for (c1,c2) in n1.chars().rev().zip(n2.chars().rev().chain(std::iter::repeat('0'))) {
            let n = c1.to_digit(10).unwrap() + c2.to_digit(10).unwrap() + carry;
            out.push((n % 10).to_string());
            carry = n / 10;
        }
        if carry > 0 {
            out.push(carry.to_string());
        }
        out.iter().rev().map(|x| x.to_string()).collect()
    }
}