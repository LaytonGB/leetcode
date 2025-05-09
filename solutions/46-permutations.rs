impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let factorial = (2..=n).product();
        let mut res = Vec::with_capacity(factorial);
        Self::perm(&mut res, &nums, n, &mut Vec::with_capacity(n));
        res
    }

    fn perm(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, remaining: usize, temp: &mut Vec<i32>) {
        if remaining == 0 {
            res.push(temp.clone());
            return;
        }

        for n in nums.iter() {
            if !temp.contains(n) {
                temp.push(*n);
                Self::perm(res, nums, remaining - 1, temp);
                temp.pop();
            }
        }
    }
}