#[derive(Debug, Clone)]
struct TrieNode {
    num: usize,
    conn: [Option<Box<Self>>; 10],
}

impl TrieNode {
    fn new() -> Self {
        Self {
            num: usize::MAX,
            conn: std::array::from_fn(|_| None),
        }
    }
    
    fn merge(&mut self, mut other: Self) {
        for i in 0..10 {
            match (&mut self.conn[i], other.conn[i].take()) {
                (None, None) | (Some(_), None) => {}
                (None, Some(conn)) => self.conn[i] = Some(conn),
                (Some(a), Some(b)) => a.merge(*b),
            }
        }
    }
}

impl TryFrom<usize> for TrieNode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > 9 {
            return Err(());
        }

        let mut res = Self::new();
        res.num = value;
        Ok(res)
    }
}

impl TryFrom<i32> for TrieNode {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(());
        }
        
        let nums = get_rev_digit_vec(value);

        let mut last_num = 0;
        let mut last: Option<Self> = None;
        for n in nums {
            let mut node = Self::try_from(n)?;
            node.conn[last_num] = last.map(|l| Box::new(l));

            last_num = n;
            last = Some(node);
        }

        let mut res = Self::new();
        res.conn[last_num] = last.map(|l| Box::new(l));
        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn find_longest_match_len(&self, values: &[i32]) -> Option<i32> {
        values.iter()
            .map(|&v| get_rev_digit_vec(v))
            .map(|nums| Self::dfs_longest_match_len(&self.root, &nums, nums.len() - 1))
            .max()
    }

    fn dfs_longest_match_len(node: &TrieNode, nums: &[usize], i: usize) -> i32 {
        if i > nums.len() {
            return 0;
        }
        let last = nums[i];
        let Some(next) = &node.conn[last] else {
            return 0;
        };
        return Self::dfs_longest_match_len(next, nums, i - 1) + 1;
    }
}

impl From<&[i32]> for Trie {
    fn from(value: &[i32]) -> Self {
        Self {
            root: value.iter().rev()
                .filter_map(|&x| TrieNode::try_from(x).ok())
                .fold(TrieNode::new(), |mut res, x| {
                    res.merge(x);
                    res
                })
        }
    }
}

fn get_rev_digit_vec(mut value: i32) -> Vec<usize> {
    let mut res = vec![];
    while value > 0 {
        res.push((value % 10) as usize);
        value /= 10;
    }
    res
}

impl Solution {
    pub fn longest_common_prefix(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let trie = Trie::from(a.as_slice());
        trie.find_longest_match_len(&b).unwrap() as i32
    }
}