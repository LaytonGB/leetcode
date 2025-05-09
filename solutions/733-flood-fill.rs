use std::collections::HashSet;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr,sc) = (sr as usize, sc as usize);
        if image[sr][sc] == color { return image; }
        let mut stack = Vec::from([(sr,sc)]);
        let mut visited = HashSet::from([(sr,sc)]);
        while let Some((r,c)) = stack.pop() {
            for (a,b) in [(r-1,c), (r,c-1), (r+1,c), (r,c+1)] {
                if a < 0 || a >= image.len() || b < 0 || b >= image[0].len() { continue; }
                if !visited.contains(&(a,b)) && image[a][b] == image[r][c] {
                    stack.push((a,b));
                    visited.insert((a,b));
                }
            }
        }
        // println!("{:?}", visited);
        for (r,c) in visited.drain() {
            image[r][c] = color;
        }
        image
    }
}