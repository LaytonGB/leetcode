use std::collections::VecDeque;

impl Solution {
    pub fn count_students(children: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut looped = 0;
        let mut children = VecDeque::from(children);
        let mut sandwiches = VecDeque::from(sandwiches);
        println!("{:?}", children);
        while let Some(&s) = sandwiches.front() {
            if let Some(c) = children.pop_front() {
                println!("{:?}", children);
                if c == s {
                    looped = 0;
                    sandwiches.pop_front();
                } else if looped >= children.len() {
                    return children.len() as i32 + 1;
                } else {
                    looped += 1;
                    children.push_back(c);
                }
            } else {
                return 0;
            }
        }
        children.len() as i32
    }
}