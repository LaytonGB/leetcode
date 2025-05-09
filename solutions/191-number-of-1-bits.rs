impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        // u32::count_ones(n) as i32
        let mut res = 0_i32;
        while n > 0 {
            res += 1;
            n &= (n - 1);
        }
        res
    }
}