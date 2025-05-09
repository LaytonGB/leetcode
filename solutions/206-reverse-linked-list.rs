impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut boxed) = curr {
            let next = boxed.next.take();
            boxed.next = prev;
            prev = Some(boxed);
            curr = next;
        }
        prev
    }
}