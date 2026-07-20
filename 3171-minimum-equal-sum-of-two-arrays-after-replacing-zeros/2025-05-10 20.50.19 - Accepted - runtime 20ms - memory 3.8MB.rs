impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (l_sum, l_zeroes) = nums1.iter().fold((0, 0), Self::acc_sum_and_zero_count);
        let (r_sum, r_zeroes) = nums2.iter().fold((0, 0), Self::acc_sum_and_zero_count);

        if l_sum == r_sum {
            // Valid because we've already made all zeroes into ones.
            return l_sum as i64;
        } else if l_zeroes == 0 && l_sum < r_sum
        || r_zeroes == 0 && l_sum > r_sum {
            return -1;
        }

        (l_sum).max(r_sum)
    }

    fn acc_sum_and_zero_count((mut sum, mut zero_count): (i64, i64), value: &i32) -> (i64, i64) {
        if *value == 0 {
            sum += 1;
            zero_count += 1;
        }
        // Zeroes give `sum` a +1 because ALL zeroes must be given a >zero value.
        // If we account for this now, we can do less math later.
        ((sum + *value as i64), zero_count)
    }
}