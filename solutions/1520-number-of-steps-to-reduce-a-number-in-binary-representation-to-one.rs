// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/solutions/5223591/rust-with-iterator
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut bytes = s.bytes()
            .enumerate()
            .rev()
            .skip_while(|&(_, b)| b == b'0');

        if bytes.next() == Some((0, b'1')) {
            return s.len() as i32 - 1;
        }
        
        1 + (s.len() + bytes.filter(|&(_, b)| b == b'0').count()) as i32
    }
}