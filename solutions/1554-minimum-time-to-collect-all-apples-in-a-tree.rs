use std::collections::{HashMap,HashSet};

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let map = Self::adj_lis(edges);
        // println!("map: {map:?}");
        let mut visited = HashSet::new();
        Self::dfs(0, 0, &map, &has_apple, &mut visited)
    }

    fn adj_lis(edges:Vec<Vec<i32>>) -> HashMap<i32,Vec<i32>> {
        edges.into_iter().fold(HashMap::new(), |mut map, e| {
            map.entry(e[0]).or_insert(Vec::new()).push(e[1]);
            map.entry(e[1]).or_insert(Vec::new()).push(e[0]);
            map
        })
    }

    fn dfs(node:i32, cost:i32, map:&HashMap<i32,Vec<i32>>, apples:&Vec<bool>, visited:&mut HashSet<i32>) -> i32 {
        if visited.contains(&node) {return 0;}
        visited.insert(node);
        let child_costs = map.get(&node).unwrap_or(&Vec::new()).iter()
            .map(|c| Self::dfs(*c, 2, map, apples, visited)).sum::<i32>();
        if child_costs == 0 && apples[node as usize] == false {0}
        else {cost + child_costs}
    }
}