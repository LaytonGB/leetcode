use std::collections::HashMap;
impl Solution {
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {        
        let mut vals = Vec::new();
        while let Some(node) = head {
            vals.push(node.val);
            head = node.next;
        }

        let mut sum = 0;
        let mut sums = HashMap::from([(0, -1)]);
        let mut skip = vec![false; vals.len()];
        for i in 0..vals.len() {
            sum += vals[i];
            
            if let Some(j) = sums.get(&sum) {
                let mut temp_sum = sum;
                for k in (j + 1) as usize..=i {
                    temp_sum += vals[k];
                    sums.remove(&temp_sum);
                    skip[k] = true;
                }
            }

            sums.insert(sum, i as i32);
        }

        vals.drain(..)
            .enumerate()
            .filter(|(i, _)| !skip[*i])
            .map(|(_, x)| {
                Some(Box::new(ListNode::new(x)))
            })
            .rev()
            .fold(None, |last_node, mut node| {
                node.as_mut().unwrap().next = last_node;
                node
            })
    }
}