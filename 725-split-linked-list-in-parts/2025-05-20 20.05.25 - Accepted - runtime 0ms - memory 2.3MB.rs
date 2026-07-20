// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;

        let n = Self::get_length(&head);

        if k == 0 || n == 0 {
            return vec![None; k];
        }

        let part_size_min = n / k;
        let mut remainder = (n % k) as i32;

        let mut res = vec![None; k];

        for i in 0..k {
            (res[i], head) = Self::take_n(head, part_size_min + (remainder > 0) as usize);
            remainder -= 1;
        }

        res
    }

    fn get_length(head: &Option<Box<ListNode>>) -> usize {
        let mut node = head;
        let mut res = 0;
        while let Some(n) = node.as_ref() {
            res += 1;
            node = &n.next;
        }
        res
    }

    fn take_n(mut head: Option<Box<ListNode>>, n: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        if n == 0 {
            return (head, None);
        }
        
        let mut a = &mut head;

        for i in 0..n - 1 {
            a = &mut a.as_mut().unwrap().next;
        }

        let rest = a.as_mut().unwrap().next.take();

        (head, rest)
    }
}