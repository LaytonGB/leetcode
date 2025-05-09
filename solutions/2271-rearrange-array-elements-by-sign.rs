impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() / 2;
        let mut pos = Vec::with_capacity(n);
        let mut neg = Vec::with_capacity(n);
        let mut res = Vec::with_capacity(n * 2);
        
        for x in nums.into_iter() {
            if x > 0 {
                pos.push(x);
            } else {
                neg.push(x);
            }
        }

        for i in 0..n {
            res.push(pos[i]);
            res.push(neg[i]);
        }

        res
    }
}