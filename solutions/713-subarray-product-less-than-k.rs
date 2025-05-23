impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        
        let mut res = 0;
        let mut prod = 1;
        
        let mut l = 0;
        for r in 0..nums.len() {
            prod *= nums[r];

            while prod >= k {
                prod /= nums[l];
                l += 1;
            }

            res += r - l + 1;
        }

        res as i32
    }
}