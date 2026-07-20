impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        
        let l = nums.iter().fold(Vec::with_capacity(n), |mut l, x| {
            l.push(l.last().unwrap_or(&0) + x);
            l
        });
        println!("{:?}", l);
        let r = nums.iter().rev().fold(Vec::with_capacity(n), |mut r, x| {
            r.push(r.last().unwrap_or(&0) + x);
            r
        });
        println!("{:?}", r);

        (0..n).map(|i| (l[i] - r[n - i - 1]).abs() as i32)
            .collect()
    }
}