impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort_unstable();
        Self::backtrack(&mut res, &mut Vec::new(), &nums);
        res
    }

    fn backtrack(res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &[i32]) {
        res.push(temp.clone());
        for (i, &x) in nums.iter().enumerate() {
            temp.push(x);
            Self::backtrack(res, temp, &nums[i + 1..]);
            temp.pop();
        }
    }
}