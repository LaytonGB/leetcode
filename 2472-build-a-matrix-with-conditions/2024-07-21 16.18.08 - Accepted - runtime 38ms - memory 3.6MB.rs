// topological sort
// based on https://leetcode.com/problems/build-a-matrix-with-conditions/solutions/5510063/explanations-no-one-will-give-you-very-detailed-approach-extremely-simple-and-effective

use std::iter::repeat;

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (Some(row_sorting), Some(col_sorting)) = (
            Self::topo_sort(row_conditions, k),
            Self::topo_sort(col_conditions, k)
        ) else {
            return vec![];
        };

        let value_position = row_sorting.into_iter().enumerate().zip(repeat(0_usize))
            .chain(col_sorting.into_iter().enumerate().zip(repeat(1_usize)))
            .fold(vec![[0, 0]; k], |mut res, ((pos, row), col)| {
                res[row as usize - 1][col] = pos;
                res
            });

        value_position.into_iter()
            .enumerate()
            .fold(vec![vec![0; k]; k], |mut res, (i, [r, c])| {
                res[r][c] = i as i32 + 1;
                res
            })
    }

    fn topo_sort(edges: Vec<Vec<i32>>, k: usize) -> Option<Vec<i32>> {
        let graph = edges.into_iter()
            .fold(vec![vec![]; k], |mut m, e| {
                let (l, h) = (e[0], e[1]);
                m[l as usize - 1].push(h);
                m
            });

        let mut res = Vec::with_capacity(k);
        let mut visited = vec![false; k];
        let mut path = Vec::with_capacity(k + 1); // extra space to avoid re-allocation if the graph is fully cyclic

        let cycle_exists = (1..=k as i32)
            .any(|n| !Self::dfs(n, &graph, &mut visited, &mut path, &mut res));
        if cycle_exists {
            return None;
        }

        res.reverse();
        Some(res)
    }

    fn dfs(
        node: i32,
        graph: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        path: &mut Vec<i32>, // used for cycle detection
        res: &mut Vec<i32>, // stores reversed path, kinda
    ) -> bool {
        if path.contains(&node) {
            return false;
        }

        if visited[node as usize - 1] {
            return true;
        }

        visited[node as usize - 1] = true;
        path.push(node);
        if graph[node as usize - 1].iter().any(|&n| !Self::dfs(n, graph, visited, path, res)) {
            return false;
        }
        path.pop();
        res.push(node);

        true
    }
}