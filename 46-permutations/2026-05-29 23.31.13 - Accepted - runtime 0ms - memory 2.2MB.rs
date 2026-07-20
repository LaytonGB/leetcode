impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::h(&nums, &mut res, &mut vec![]);
        res
    }

    fn h(nums: &[i32], res: &mut Vec<Vec<i32>>, curr: &mut Vec<i32>) {
        if curr.len() == nums.len() {
            res.push(curr.clone());
            return;
        }

        for n in nums {
            if !curr.contains(n) {
                curr.push(*n);
                Self::h(nums, res, curr);
                curr.pop();
            }
        }
    }
}