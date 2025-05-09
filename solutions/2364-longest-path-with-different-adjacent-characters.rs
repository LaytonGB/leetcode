impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let adj = (1..n).fold(vec![Vec::new(); n], |mut v, i| {
            v[parent[i] as usize].push(i);
            v
        });
        let chars:Vec<usize> = s.into_bytes().into_iter()
            .map(|b| (b - b'a') as usize).collect();
        let mut res = 1;
        Self::dfs(0, &adj, &chars, &mut res);
        res
    }

    fn dfs(n:usize, adj:&Vec<Vec<usize>>, chars:&Vec<usize>, res:&mut i32) -> i32 {
        let (a,b) = adj[n].iter().fold((0,0), |(t1, t2), m| {
            let res = Self::dfs(*m, adj, chars, res);
            if chars[n] == chars[*m] {return (t1,t2);}
            if res > t1 {(res,t1)}
            else if res > t2 {(t1,res)}
            else {(t1,t2)}
        });
        *res = (*res).max(a + b + 1);
        a + 1
    }
}