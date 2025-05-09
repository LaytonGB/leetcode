impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // taken from fastest rust answer
        let mut count = 0;
        let mut candidate = 0;
        for x in nums {
            if count == 0 {
                candidate = x;
            }
            if x == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}