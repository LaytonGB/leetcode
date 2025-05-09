impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        match a % b {
            0 => b,
            c => Self::gcd(b, c),
        }
    }
    
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(a) = cur {
            if let Some(b) = &a.next {
                let gcd = Self::gcd(a.val, b.val);
                let c = Box::new(ListNode {val: gcd, next: a.next.take()});
                cur = &mut a.next.insert(c).next;
            } else {
                break;
            }
        }
        head
    }
}
