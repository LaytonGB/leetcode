impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut q = queries.iter()
            .fold(vec![0_i32; nums.len() + 1], |mut acc, q| {
                let (l, r) = (q[0] as usize, q[1] as usize);
                acc[l] += 1;
                acc[r + 1] -= 1;
                acc
            });
        
        let mut q_sum = 0;
        for (n, q) in nums.iter().zip(q.iter()) {
            q_sum += q;
            if *n > q_sum {
                return false;
            }
        }

        true
    }
}