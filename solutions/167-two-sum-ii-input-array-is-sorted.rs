impl Solution {
    pub fn two_sum(n: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut h) = (0, n.len()-1);
        let mut sum;
        while l < h {
            sum = n[l] + n[h];
            if sum == target {
                return vec![l as i32 + 1, h as i32 + 1];
            } else if sum > target {
                h -= 1;
            } else {
                l += 1;
            }
        }
        unreachable!()
    }
}