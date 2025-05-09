impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::h(&nums)
    }

    fn h(nums: &[i32]) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums.to_vec();
        }

        let m = nums.len() / 2;
        let a = Self::h(&nums[..m]);
        let b = Self::h(&nums[m..]);

        let mut res = Vec::with_capacity(nums.len());

        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            if a[i] <= b[j] {
                res.push(a[i]);
                i += 1;
            } else {
                res.push(b[j]);
                j += 1;
            }
        }

        while i < a.len() {
            res.push(a[i]);
            i += 1;
        }
        while j < b.len() {
            res.push(b[j]);
            j += 1;
        }

        res
    }
}