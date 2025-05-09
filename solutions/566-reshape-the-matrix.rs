impl Solution {
    pub fn matrix_reshape(mut mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let (r, c) = (r as usize, c as usize);
        let size = m * n;
        if size != r * c { return mat; }

        let mut res = vec![Vec::with_capacity(c); r];
        let mut i = 0;
        mat.drain(..).flatten().for_each(|x| { res[i / c].push(x); i += 1 });
        res
    }
}