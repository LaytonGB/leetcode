impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let tree = connections.iter().fold(vec![Vec::new(); n], |mut t, r| {
            let (a, b) = (r[0], r[1]);
            t[a as usize].push(b);
            t[b as usize].push(-a);
            t
        });
        // println!("{tree:?}");
        Self::dfs(&tree, 0, 0)
    }

    fn dfs(tree: &Vec<Vec<i32>>, last: i32, node: i32) -> i32 {
        tree[node as usize]
            .iter()
            .fold(0, |sum, &n| {
                let nabs = n.abs();
                if nabs == last {
                    sum
                } else if nabs == n { // was positive
                    sum + 1 + Self::dfs(tree, node, nabs)
                } else {
                    sum + Self::dfs(tree, node, nabs)
                }
            })
    }
}