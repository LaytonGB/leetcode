impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut a = Vec::new();
        let mut fo = head;
        while let Some(f) = fo {
            a.push(f.val);
            let f = f.next.unwrap();
            a.push(f.val);
            fo = f.next;
        }
        let mut res = -1;
        for i in 0 .. a.len() / 2 {
            res = res.max(a[i] + a[a.len() - 1 - i]);
        }
        res
    }
}