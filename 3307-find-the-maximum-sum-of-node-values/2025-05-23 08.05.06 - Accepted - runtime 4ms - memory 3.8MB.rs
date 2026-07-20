impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let flipped: Vec<i32> = nums.iter().map(|x| x ^ k).collect();
        let mut zip_iter = nums.iter().zip(flipped.iter());
        let want_to_flip = zip_iter.clone()
            .filter(|(n, f)| f > n)
            .count();
        let best_sum = zip_iter.clone()
            .map(|(n, f)| *n.max(f) as i64)
            .sum::<i64>();
        
        if want_to_flip % 2 == 1 {
            best_sum - zip_iter.map(|(n, f)| (n - f).abs() as i64)
                .min()
                .unwrap()
        } else {
            best_sum
        }
    }
}