impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut r, mut w, mut b) = (0, 0, nums.len() - 1);
        while w <= b {
            match nums[w] {
                0 => {
                    nums.swap(w, r);
                    w += 1;
                    r += 1;
                }
                1 => {
                    w += 1;
                }
                2 => {
                    nums.swap(w, b);
                    let Some(new_b) = b.checked_sub(1) else { return; };
                    b = new_b;
                }
                _ => unreachable!(),
            }
        }
    }
}