use std::cmp::Ordering as O;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (l_sum, l_zeroes) = nums1.iter().fold((0, 0), Self::acc_sum_and_zero_count);
        let (r_sum, r_zeroes) = nums2.iter().fold((0, 0), Self::acc_sum_and_zero_count);

        match (l_sum.cmp(&r_sum), l_zeroes, r_zeroes) {
            (O::Less, 0, _) | (O::Greater, _, 0) => {
                return -1;
            }
            _ => {}
        }

        (l_sum).max(r_sum)
    }

    fn acc_sum_and_zero_count((sum, zero_count): (i64, i64), value: &i32) -> (i64, i64) {
        let is_zero = (*value == 0) as i64;
        // Zeroes give `sum` a +1 because ALL zeroes must be given a >zero value.
        // If we account for this now, we can do less math later.
        ((sum + *value as i64) + is_zero, zero_count + is_zero)
    }
}