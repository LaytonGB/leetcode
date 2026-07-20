impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while !nums.is_sorted() {
            let (pos, sum) = nums.windows(2)
                .map(|w| w[0] + w[1])
                .enumerate()
                .min_by_key(|(_, x)| *x)
                .unwrap();
            
            nums[pos] = sum;
            nums.remove(pos + 1);

            i += 1;
        }

        i
    }
}