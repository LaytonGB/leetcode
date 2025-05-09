impl Solution {
    pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(h) = head {
            head = Self::r(h).0;
        }
        head
    }

    fn r(mut head: Box<ListNode>) -> (Option<Box<ListNode>>, i32) {
        let (next, r_max) = head.next.map(|n| Self::r(n)).unwrap_or((None, 1));

        if head.val < r_max {
            (next, r_max)
        } else {
            let v = head.val;
            head.next = next;
            (Some(head), v)
        }
    }
}