impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == target {
                0
            } else {
                -1
            };
        }
        // no rotation case
        if nums[0] < nums[nums.len() - 1] {
            return nums.binary_search(&target).map(|i| i as i32).unwrap_or(-1);
        }
        
        let (mut l, mut h) = (0, nums.len() - 1);
        
        // find middle
        while l < h {
            let m = l + (h - l) / 2;
            if nums[m] > nums[h] {
                l = m + 1;
            } else {
                h = m;
            }
        }

        // reduce search area
        let mid = l;
        if target <= nums[nums.len() - 1] {
            l = mid;
            h = nums.len() - 1;
        } else {
            l = 0;
            h = mid - 1;
        }

        // find target
        while l < h {
            let m = l + (h - l) / 2;
            if target > nums[m] {
                l = m + 1;
            } else {
                h = m;
            }
        }

        if nums[l] == target {
            l as i32
        } else {
            -1
        }
    }
}