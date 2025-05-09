impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut found = [(false, false); 1001];
        let mut res = -1;
        for mut n in nums.into_iter() {
            match n {
                ..=0 => {
                    let n = -n as usize;
                    found[n].0 = true;
                    if found[n].1 {
                        res = res.max(n as i32);
                    }
                }
                1.. => {
                    let n = n as usize;
                    found[n].1 = true;
                    if found[n].0 {
                        res = res.max(n as i32);
                    }
                }
            };
        }
        res
    }
}