impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() < n - 1 {
            return -1;
        }
        let mut map: Vec<Vec<usize>> = connections.iter().fold(vec![Vec::new(); n], |mut m, cons| {
            let (a, b) = (cons[0] as usize, cons[1] as usize);
            m[a].push(b);
            m[b].push(a);
            m
        });
        // println!("{:?}", map);
        let mut res = 0;
        let mut visited = vec![false; n];
        for c in 0..n {
            if !visited[c] {
                Self::dfs(&map, &mut visited, c);
                res += 1;
            }
        }
        res - 1
    }

    fn dfs(
        map: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        start: usize,
    ) {
        visited[start] = true;
        for c in map[start].iter() {
            if !visited[*c] {
                Self::dfs(map, visited, *c);
            }
        }
    }
}
