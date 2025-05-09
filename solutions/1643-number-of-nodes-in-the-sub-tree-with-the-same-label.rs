impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut map = vec![Vec::new(); n];
        for e in edges {
            let (a,b) = (e[0] as usize, e[1] as usize);
            map[a].push(b);
            map[b].push(a);
        }
        let labels = labels.into_bytes().into_iter()
            .map(|b| (b - b'a') as usize).collect::<Vec<usize>>();
        let mut label_counts = [0; 26]; // one per char because of lower-case ascii letter labelling
        let mut res = vec![0; n];
        Self::dfs(0, n, &map, &labels, &mut label_counts, &mut res);
        res
    }

    fn dfs(
        n:usize, 
        parent:usize,
        map:&Vec<Vec<usize>>,
        labels:&Vec<usize>, 
        label_counts:&mut [usize],
        res:&mut Vec<i32>
    ) {
        let i = labels[n];
        let temp = label_counts[i];
        label_counts[i] = 1;

        map[n].iter().for_each(|m| {
            if *m != parent {
                Self::dfs(*m, n, map, labels, label_counts, res);
            }
        });

        res[n] = label_counts[i] as i32;
        label_counts[i] += temp;
    }
}