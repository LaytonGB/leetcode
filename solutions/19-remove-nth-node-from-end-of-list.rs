use std::mem;
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        let mut curr = &head;
        while let Some(next) = curr {
            stack.push(next.clone());
            curr = &stack.last().unwrap().next;
        }

        let mut last = None;
        while n > 1 {
            let mut next_last = stack.pop();
            if let Some(nl) = next_last.as_mut() {
                nl.next = last;
            }

            last = next_last;

            n -= 1;
        }

        stack.pop();

        while let Some(mut nl) = stack.pop() {
            nl.next = last;
            last = Some(nl);
        }

        last
    }
}