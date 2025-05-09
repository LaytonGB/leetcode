impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // head is an iterator
        // construct using head's nodes, storing only when tallied
        let mut res = None; // empty at first, filled when tallied
        let mut curr = &mut res;

        while let Some(mut node) = head {
            let mut sum = node.val;

            loop {
                node = node.next.unwrap();

                if node.val == 0 {
                    break;
                }

                sum += node.val;
            }

            head = node.next.take();
            node.val = sum;
            curr = &mut curr.insert(node).next;
        }

        res
    }
}