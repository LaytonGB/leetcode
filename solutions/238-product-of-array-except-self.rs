impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prod = vec![1; n];
        for i in 1..n {
            prod[i] = nums[i - 1] * prod[i - 1];
        }

        let mut r_prod = 1;
        for i in (0..n).rev() {
            prod[i] *= r_prod;
            r_prod *= nums[i];
        }

        prod
    }
}