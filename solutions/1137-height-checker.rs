impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();

        heights.into_iter()
            .zip(expected.into_iter())
            .fold(0, |res, (h, e)| res + (h != e) as i32)
    }
}