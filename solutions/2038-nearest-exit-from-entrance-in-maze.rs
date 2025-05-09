use std::collections::HashSet;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut steps: i32 = 0;
        let mut curr: Vec<(i32, i32)> = vec![(entrance[0], entrance[1])];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((entrance[0], entrance[1]));
        loop {
            let mut next_curr = Vec::new();
            while let Some(coords) = curr.pop() {
                let (r, c) = coords;
                // if exit reached
                let top_bound = r == 0;
                let bot_bound = r == maze.len() as i32 - 1;
                let lef_bound = c == 0;
                let rig_bound = c == maze[0].len() as i32 - 1;
                if (steps != 0 && (top_bound || bot_bound || lef_bound || rig_bound)) {
                    return steps;
                }
                for (y,x) in dirs {
                    if (y == -1 && top_bound || y == 1 && bot_bound || x == -1 && lef_bound || x == 1 && rig_bound) {
                        continue;
                    }
                    let (nr, nc) = (r+y, c+x);
                    // else if visitable
                    if (!visited.contains(&(nr, nc)) && maze[nr as usize][nc as usize] != '+') {
                        visited.insert((nr, nc));
                        next_curr.push((nr, nc));
                    }
                }
            }
            if (next_curr.len() == 0) {
                return -1;
            }
            steps += 1;
            curr = next_curr;
        }
        -1
    }
}