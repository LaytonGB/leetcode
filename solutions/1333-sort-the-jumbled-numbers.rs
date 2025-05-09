impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut sort = nums.into_iter().map(|n| {
            if n == 0 {
                return (mapping[0], n);
            }
            
            let mut m = 0_i32;
            for i in 0..=n.ilog10() {
                m += mapping[((n / 10_i32.pow(i)) % 10) as usize] * 10_i32.pow(i);
            }
            (m, n)
        }).collect::<Vec<_>>();

        sort.sort_unstable_by_key(|x| x.0);
        
        sort.into_iter().map(|(s, n)| n).collect()
    }
}