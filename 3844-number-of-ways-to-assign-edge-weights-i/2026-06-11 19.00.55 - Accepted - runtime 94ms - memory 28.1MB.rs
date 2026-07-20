// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i/solutions/8326725/1-by-gevajat389-cxku

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;
        
        let tree = edges.into_iter()
            .fold(vec![Vec::with_capacity(2); n + 1], |mut t, x| {
                let &[u, v] = &x[..] else { unreachable!() };
                t[u as usize].push(v as usize);
                t[v as usize].push(u as usize);
                t
            });

        let depth = Self::dfs(&tree, 1, 0);

        Self::pow_mod(2, depth - 1) as i32
    }

    // Without the linked solution, I absolutely would've assumed this is too slow
    fn dfs(tree: &Vec<Vec<usize>>, node: usize, prev: usize) -> usize {
        tree[node].iter()
            .filter(|&&c| c != prev)
            .map(|&c| Self::dfs(tree, c, node) + 1)
            .max()
            .unwrap_or(0)
    }

    // Gotta mod as we pow
    fn pow_mod(mut base: usize, mut exp: usize) -> usize {
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % MOD;
            }
            base = (base * base) % MOD;
            exp >>= 1;
        }
        res
    }
}