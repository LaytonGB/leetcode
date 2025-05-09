impl Solution {
    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(
            ListNode {
                val: 0,
                next: head,
            }
        ));
        let Some(mut prev) = head.as_mut() else {
            return None;
        };

        while let Some(next) = prev.next.as_mut() {
            let double = next.val * 2;
            prev.val += double / 10;
            next.val = double % 10;
            prev = next;
        }

        if head.as_ref().is_some_and(|h| h.val == 0) {
            head = head.unwrap().next;
        }

        head
    }
}