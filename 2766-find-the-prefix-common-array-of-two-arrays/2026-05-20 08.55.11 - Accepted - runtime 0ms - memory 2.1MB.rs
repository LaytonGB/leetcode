impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut one = [false; 50];
        let mut count = 0;
        let mut res = vec![0; n];
        for i in 0..n {
            match one[a[i] as usize - 1] {
                true => count += 1,
                false => one[a[i] as usize - 1] = true,
            }
            match one[b[i] as usize - 1] {
                true => count += 1,
                false => one[b[i] as usize - 1] = true,
            }
            res[i] = count;
        }
        res
    }
}