impl Solution {
    pub fn maximum_energy(mut energy: Vec<i32>, k: i32) -> i32 {
        unsafe{
            let k = k as usize;
            let n = energy.len();
            let mut res = *energy.last().unwrap_unchecked();
            for i in (0..energy.len()).rev() {
                if i + k < n {
                    energy[i] += energy[i+k];
                }
                res = res.max(energy[i]);
            }
            res
        }
    }
}