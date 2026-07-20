impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();

        let [a, b, c] = &nums[..] else { unreachable!() };

        if a + b <= *c {
            "none".to_string()
        } else if a == b {
            if b == c {
                "equilateral".to_string()
            } else {
                "isosceles".to_string()
            }
        } else if a == c || b == c {
            "isosceles".to_string()
        } else {
            "scalene".to_string()
        }
    }
}