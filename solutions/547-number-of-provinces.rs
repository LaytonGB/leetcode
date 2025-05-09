impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut v = vec![false; is_connected.len()];
        let mut res = 0;
        for i in 0 .. is_connected.len() {
            if !v[i] {
                res += 1;
                Self::dfs(&mut v, &is_connected, i);
            }
        }
        res
    }

    fn dfs(v: &mut Vec<bool>, m: &Vec<Vec<i32>>, i: usize) {
        v[i] = true;
        for (j,&n) in m[i].iter().enumerate() {
            if n == 1 && !v[j as usize] {
                Self::dfs(v, m, j as usize);
            }
        }
    }
}