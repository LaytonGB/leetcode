impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let lim = nums.len() / 3;
        
        let (mut candidate1, mut count1) = (0, 0);
        let (mut candidate2, mut count2) = (1, 0);
        for &x in nums.iter() {
            if x == candidate1 {
                count1 += 1;
            } else if x == candidate2 {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = x;
                count1 = 1;
            } else if count2 == 0 {
                candidate2 = x;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        let mut res = Vec::with_capacity(2);
        count1 = nums.iter().filter(|x| **x == candidate1).count();
        count2 = nums.iter().filter(|x| **x == candidate2).count();
        if count1 > lim {
            res.push(candidate1);
        }
        if count2 > lim {
            res.push(candidate2);
        }

        res
    }
}