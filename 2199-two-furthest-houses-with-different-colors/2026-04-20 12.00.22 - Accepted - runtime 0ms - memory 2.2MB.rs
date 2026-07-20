impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        
        let l = (0..n)
            .find(|&i| colors[n-1] != colors[i])
            .unwrap();

        let r = (0..n).rev()
            .find(|&i| colors[0] != colors[i])
            .unwrap();
            
        r.max(n-1-l) as i32
    }
}