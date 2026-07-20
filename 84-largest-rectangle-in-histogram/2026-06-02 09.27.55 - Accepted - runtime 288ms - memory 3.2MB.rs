impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(heights.len());
        let mut res = 0;
        for (i, &h) in heights.iter().enumerate() {
            let mut w = 1;
            while let Some(&(h2, w2)) = stack.last() {
                if h2 >= h {
                    w += w2;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((h, w));

            let mut w = 0;
            for (h, w2) in stack.iter().rev() {
                w += w2;
                res = res.max(h * w);
            }
        }
        res
    }
}