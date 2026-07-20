impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let k = k as usize;
        let mut s = vec!['0'];
        while s.len() < k {
            let mut rev = s.iter().map(|&x| if x == '0' { '1' } else { '0' }).rev().collect();
            s.push('1');
            s.append(&mut rev);
        }
        s[k - 1]
    }
}