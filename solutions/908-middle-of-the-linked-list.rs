impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            if fast.is_some() {
                slow = &slow.as_ref().unwrap().next;
                fast = &fast.as_ref().unwrap().next;
            } else {
                break;
            }
        }
        slow.clone()
    }
}