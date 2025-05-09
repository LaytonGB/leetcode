impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two(l1.as_ref(), l2.as_ref(), 0)
    }

    fn add_two(l1: Option<&Box<ListNode>>, l2: Option<&Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let val = n1.val + n2.val + carry;
                let next = Self::add_two(n1.next.as_ref(), n2.next.as_ref(), val / 10);

                Some(Box::new(ListNode {
                    val: val % 10,
                    next,
                }))
            }
            (Some(n), _) | (_, Some(n)) => {
                let val = n.val + carry;
                let next = Self::add_two(n.next.as_ref(), None, val / 10);

                Some(Box::new(ListNode {
                    val: val % 10,
                    next,
                }))
            }
            _ if carry == 0 => None,
            _ => Some(Box::new(ListNode::new(carry))),
        }
    }
}