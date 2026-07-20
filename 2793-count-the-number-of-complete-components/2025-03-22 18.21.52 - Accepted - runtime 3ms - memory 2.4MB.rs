impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut parents = edges.iter().fold((0..n).collect::<Vec<usize>>(), |mut res, e| {
            let (a, b) = (e[0] as usize, e[1] as usize);
            let (pa, pb) = (Self::get_united(&mut res, a), Self::get_united(&mut res, b));
            res[pa] = pb;
            res
        });

        let mut node_counts = (0..n).fold(vec![0; n], |mut res, i| {
            res[Self::get_united(&mut parents, i)] += 1;
            res
        });

        let mut edge_counts = edges.into_iter().fold(vec![0; n], |mut res, e| {
            res[Self::get_united(&mut parents, e[0] as usize)] += 1;
            res
        });

        (0..n).map(|i| (i, node_counts[i], edge_counts[i]))
            .filter(|(_, node_count, _)| *node_count != 0)
            .fold(0_i32, |res, (i, node_count, edge_count)| {
                res + (edge_count == node_count * (node_count - 1) / 2) as i32
            })
    }

    fn get_united(parents: &mut [usize], node: usize) -> usize {
        if parents[node] != node {
            parents[node] = Self::get_united(parents, parents[node]);
        }
        parents[node]
    }
}