impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut root: [usize; 26] = [usize::default(); 26];

        let mut ans = String::new();
        // init root. initialy each element is in its own group.
        for i in 0..26 {
            root[i] = i as usize;
        }
        for i in 0..s1.len() {
            Self::unite(&mut root, s1.as_bytes()[i] as usize - 'a' as usize, s2.as_bytes()[i] as usize - 'a' as usize);
        }
        // build the final answer from the root element (smallest)
        for i in base_str.bytes() {
            ans.push((Self::get(&mut root, i as usize - 'a' as usize) + 'a' as usize) as u8 as char);
        }
        ans
    }

    // unite two elements
    fn unite(root: &mut [usize; 26], x: usize, y: usize) {
        // find the root of x
        let x = Self::get(root, x);
        // find the root of y
        let y = Self::get(root, y);
        // if their roots are not same, we combine them
        if x != y {
            // smaller first
            if x < y {
                root[y] = x;
            } else {
                root[x] = y;
            }
        }
    }

    // recursively get the root element
    fn get(root: &mut [usize; 26], x: usize) -> usize {
        if x == root[x] {
            return x;
        }
        root[x] = Self::get(root, root[x]);
        root[x]
    }
}
