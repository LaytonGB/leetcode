impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            h.next = Self::remove_elements(h.next, val);
            if h.val == val {
                return h.next;
            }
            return Some(h);
        }
        None
    }
}