impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let n = s.len();

        let mut s1 = Vec::with_capacity(n);
        let mut s2 = Vec::with_capacity(n);
        for c in s.chars() {
            match (c, s1.len()) {
                ('0', 0) => s2.push('0'),
                ('0', _) => s1.push('0'),
                ('1', 0) => s1.push('1'),
                ('1', _) => s2.push('1'),
                x => unreachable!("Only expected zeroes and ones, found {:?}", x),
            }
        }
        s1.into_iter().chain(s2.into_iter()).rev().collect()
    }
}