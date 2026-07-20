// https://leetcode.com/problems/block-placement-queries/solutions/8301844/reverse-processing-with-segment-tree-on-zbgsi

// SegTrees are indexed using the same methods as BinaryHeaps
// i.e. children of a node are at 2*p and 2*p+1 for the left and right nodes respectively
// Fuller explanation at VisuAlgo https://visualgo.net/en/segmenttree

use std::collections::BTreeSet;

const MAX_X: i32 = 50_000;

#[derive(Debug)]
struct SegTree {
    vals: Vec<i32>,
}

impl SegTree {
    fn new() -> Self {
        // The max size of a SegmentTree is 4 * the amount of root values
        Self { vals: vec![0; 4 * MAX_X as usize + 1] }
    }

    fn update_from<'a, 'b>(&'a mut self, mut it: impl Iterator<Item = &'b i32>) {
        let mut last = *it.next().unwrap();
        for &x in it {
            self.update(1, 0, MAX_X, x, x - last);
            last = x;
        }
    }
    
    fn update(&mut self, node: usize, l: i32, r: i32, idx: i32, val: i32) {
        // println!("node:{node} | l:{l} | r:{r} | idx:{idx} | val:{val}");
        if l == r {
            self.vals[node] = val;
            return;
        }

        // Here we use a BST to represent our barrier positions
        let mid = l + (r - l) / 2;
        if idx <= mid {
            self.update(2 * node, l, mid, idx, val);
        } else {
            self.update(2 * node + 1, mid + 1, r, idx, val);
        }

        self.vals[node] = self.vals[2 * node].max(self.vals[2 * node + 1]);
    }

    fn query(&self, node: usize, l: i32, r: i32, ql: i32, qr: i32) -> i32 {
        if ql > r || qr < l {
            return 0;
        }

        if ql <= l && qr >= r {
            return self.vals[node];
        }

        let mid = l + (r - l) / 2;

        self.query(2 * node, l, mid, ql, qr)
            .max(self.query(2 * node + 1, mid + 1, r, ql, qr))
    }

    fn remove() {}
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = queries.len();
        
        let mut q1 = queries.iter().fold(
            BTreeSet::from([0]),
            |mut q1, q| {
                match q[0] {
                    1 => { q1.insert(q[1]); }
                    2 => {}
                    _ => unreachable!(),
                };
                q1
            }
        );

        let mut st = SegTree::new();
        st.update_from(q1.iter());
        // println!("{:?}", st);

        let mut res = Vec::with_capacity(queries.len());
        for q in queries.iter().rev() {
            match q[0] {
                1 => {
                    let x = q[1];
                    let l = *q1.range(..x).next_back().unwrap();
                    let r = q1.range(x + 1..).next();

                    st.update(1, 0, MAX_X, x, 0);

                    if let Some(&r) = r {
                        st.update(1, 0, MAX_X, r, r - l);
                    }

                    q1.remove(&x);
                    // println!("removed:{} | q1:{:?}", x, q1);
                }
                2 => {
                    let x = q[1];
                    let sz = q[2];
                    // println!("x:{x} | sz:{sz}");

                    let prev = *q1.range(..=x).next_back().unwrap();
                    let best = st.query(1, 0, MAX_X, 0, prev).max(x - prev);
                    // println!("compared query:{} and {}", st.query(1, 0, MAX_X, 0, prev), x - prev);
                    // println!("did fit: {}", best >= sz);

                    res.push(best >= sz);
                }
                _ => unreachable!(),
            }
        }

        res.into_iter().rev().collect()
    }
}