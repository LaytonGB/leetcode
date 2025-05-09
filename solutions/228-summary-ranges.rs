impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return Vec::new();
        }
        
        let mut ranges = Vec::<(i32, i32)>::with_capacity(20);
        let [mut l, mut n, mut p] = [nums[0]; 3];
        for num in nums[1..].iter() {
            n = num.to_owned();
            if n != p + 1 {
                ranges.push((l, p));
                l = n;
            }
            p = n;
        }
        ranges.push((l, n));
        ranges.into_iter().map(|(l,h)| {
            if l == h {
                format!("{}", l)
            } else {
                format!("{}->{}", l, h)
            }
        }).collect()
    }
}