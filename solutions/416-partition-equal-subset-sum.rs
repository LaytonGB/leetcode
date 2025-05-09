impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 {
            return false;
        }

        let tgt = sum as usize / 2;
        
        let mut row = 1 << tgt;
        for x in nums {
            row = row | (row >> x as usize);
        }

        row & 1 == 1
    }
}