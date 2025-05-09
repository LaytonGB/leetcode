impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cache = vec![None; n];
        let (mut longest, mut count) = (0_usize, 0_i32);
        for i in 0..n {
            let (l, c) = Self::dfs(&nums, &mut cache, i);
            if l == longest {
                count += c;
            } else if l > longest {
                longest = l;
                count = c;
            }
        }
        count
    }

    fn dfs(nums: &Vec<i32>, cache: &mut Vec<Option<(usize, i32)>>, idx: usize) -> (usize, i32) {
        if let Some((l, c)) = cache[idx] {
            // println!("idx {:?} ({:?}) cache hit -> {:?}", idx, nums[idx], (l, c));
            (l, c)
        } else {
            let (mut longest, mut count) = (0_usize, 1_i32);
            let n = nums[idx];
            for i in idx+1..nums.len() {
                if n >= nums[i] {
                    continue;
                }

                let (l, c) = Self::dfs(nums, cache, i);
                if l == longest {
                    count += c;
                } else if l > longest {
                    longest = l;
                    count = c;
                }
            }
            cache[idx] = Some((longest + 1, count));
            // println!("idx {:?} ({:?}) result -> {:?}", idx, n, (longest + 1, count));
            (longest + 1, count)
        }
    }
}