const MOD: i64 = 10_i64.pow(9) + 7;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // build pascal table
        let mut pasc = Vec::with_capacity(n + 1);
        for i in 0..=n {
            pasc.push(vec![1; i + 1]);
            for j in 1..i {
                pasc[i][j] = (pasc[i-1][j-1] + pasc[i-1][j]) % MOD;
            }
        }

        // start dfs with full vec slice
        let res = Self::dfs(&nums[..], &pasc);
        (res % MOD) as i32 - 1
    }

    fn dfs(nums: &[i32], pasc: &Vec<Vec<i64>>) -> i64 {
        let n = nums.len();

        // first number is always root node, and so cannot be re-ordered
        // hence can only re-order when length > 2
        if n <= 2 {
            return 1;
        }

        // values lower than root node are on left, higher are on right
        let (left, right) = (1..n)
            .fold((Vec::new(), Vec::new()), |(mut l, mut r), i| {
                if nums[i] < nums[0] {
                    l.push(nums[i]);
                } else {
                    r.push(nums[i]);
                }
                (l, r)
            });
        
        // recursively find all reorders
        let left_res = Self::dfs(&left[..], pasc) % MOD;
        let right_res = Self::dfs(&right[..], pasc) % MOD;

        // lookup in pascal table, and combine with re-order results, modding at each stage
        (((pasc[n-1][left.len()] * left_res) % MOD) * right_res) % MOD
    }
}