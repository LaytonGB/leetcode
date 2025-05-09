use std::collections::*;
use std::cmp::Reverse;
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 || *grid.last().unwrap().last().unwrap() == 1 {
            return -1;
        } else if grid.len() == 1 {
            return 1;
        }
        let n = grid.len() as i32 - 1;
        let mut v = HashSet::new();
        let mut p = HashMap::new();
        let mut d = HashMap::from([((0, 0), 0)]);
        let mut f = BinaryHeap::from([(Reverse(0), (0, 0))]);
        while let Some((_,(r,c))) = f.pop() {
            if v.contains(&(r,c)) {
                continue;
            }
            if r == n && c == n {
                // println!("Answer found");
                return Self::path_len(p, (n, n));
            }
            v.insert((r,c));
            // println!("r:{} c:{} vis:{:?}", r, c, v);
            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    let (y, x) = (r + i, c + j);
                    if i == 0 && j == 0
                    || y < 0 || x < 0 || y > n || x > n
                    || grid[y as usize][x as usize] == 1 {
                        continue;
                    }
                    let new_dist = *d.get(&(r,c)).unwrap() + 1;
                    let priority = new_dist + (n - y).max(n - x);
                    f.push((Reverse(priority), (y, x)));
                    // replace p here if there wasnt one or the current
                    // p is better than the last p
                    d.entry((y,x))
                        .and_modify(|old_dist| if *old_dist > new_dist {
                            p.insert((y,x), (r,c));
                            *old_dist = new_dist;
                        })
                        .or_insert_with(|| {
                            p.insert((y,x), (r,c));
                            new_dist
                        });
                }
            }
            // println!("p:{:?} dist:{:?} front:{:?}", p, d, f);
        }
        -1
    }

    fn path_len(p: HashMap<(i32, i32), (i32, i32)>, mut n: (i32, i32)) -> i32 {
        // println!("PATH:{:?}", p);
        let mut res = 1;
        while n != (0, 0) {
            n = *p.get(&n).unwrap();
            res += 1;
        }
        res
    }
}