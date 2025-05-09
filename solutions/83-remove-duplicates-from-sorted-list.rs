impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return head; }
        let mut p1 = head.as_mut().unwrap();
        while let Some(p2) = p1.next.as_mut() {     // mutable so can be taken
            if p1.val == p2.val {
                p1.next = p2.next.take();
            } else {                                // only move p1 fowards if all adj duplicates removed (avoid unwrapping None)
                p1 = p1.next.as_mut().unwrap();
            }
        }
        head
    }
}