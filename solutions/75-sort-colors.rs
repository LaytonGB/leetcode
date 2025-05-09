impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l1, mut l2, mut r) = (0, 0, nums.len() - 1);
        while l2 <= r {
            match nums[l2] {
                0 => {
                    nums.swap(l1, l2);
                    l1 += 1;
                    l2 += 1;
                }
                1 => {
                    l2 += 1;
                }
                _ => {
                    nums.swap(l2, r);
                    if r == 0 {
                        break;
                    }
                    r -= 1;
                }
            }
        }
    }
}