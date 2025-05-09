impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut res = 0;
        let (mut freq1, mut freq2) = (vec![0; n + 1], vec![0; n + 1]);
        let (mut c1, mut c2) = (0, 0);
        let (mut l1, mut l2) = (0, 0);
        for r in 0..n {
            let num = nums[r] as usize;

            c1 += (freq1[num] == 0) as usize;
            c2 += (freq2[num] == 0) as usize;

            freq1[num] += 1;
            freq2[num] += 1;

            while l1 <= r && c1 > k {
                let num = nums[l1] as usize;
                freq1[num] -= 1;
                c1 -= (freq1[num] == 0) as usize;
                l1 += 1;
            }

            while l2 <= r && c2 > k - 1 {
                let num = nums[l2] as usize;
                freq2[num] -= 1;
                c2 -= (freq2[num] == 0) as usize;
                l2 += 1;
            }

            if c1 == k {
                res += l2 - l1;
            }
        }

        res as i32
    }
}