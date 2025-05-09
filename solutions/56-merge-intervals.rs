impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let mut res: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        for i in 0..intervals.len() {
            if let Some(mut last) = res.last_mut() {
                if last[1] >= intervals[i][0] {
                    last[1] = last[1].max(intervals[i][1]);
                    continue;
                }
            }

            res.push(intervals[i].clone());
        }

        res
    }
}