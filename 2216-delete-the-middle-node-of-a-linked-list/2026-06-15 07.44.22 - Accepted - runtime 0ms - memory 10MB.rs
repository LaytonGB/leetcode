impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n = Self::get_len(&head);
        let mut sentinel = ListNode {
            val: -1,
            next: head,
        };
        Self::remove_mid(&mut sentinel, n);
        sentinel.next
    }

    fn get_len(head: &Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        let mut node = head.as_ref();
        while node.is_some() {
            res += 1;
            node = node.unwrap().next.as_ref();
        }
        res
    }

    fn remove_mid(head: &mut ListNode, mut n: i32) {
        let lim = (n + 1) / 2;
        let mut node = head;
        while n > lim {
            n -= 1;
            node = node.next.as_mut().unwrap();
        }
        node.next = node.next.take().map(|n| n.next).flatten();
    }
}