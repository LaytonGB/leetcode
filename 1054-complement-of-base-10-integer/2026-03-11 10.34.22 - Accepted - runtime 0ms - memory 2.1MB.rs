impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        
        let offset = (n * 2).ilog2();
        let mut m = n;
        for i in 0..offset {
            if m & 1 == 0 {
                n += 2_i32.pow(i);
            } else {
                n -= 2_i32.pow(i);
            }
            // println!("i:{} | n:{} / {:0>32b} | m:{}", i, n, n, m);
            m >>= 1;
        }
        n
    }
}