// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/solutions/5705563/dsu-approach-with-tc-o-n-n-beats-100-in-all-languages

struct Coords {
    pub row: usize,
    pub col: usize,
}

impl From<Vec<i32>> for Coords {
    fn from(value: Vec<i32>) -> Self {
        Self {
            row: (value[0] + 1) as usize,
            col: (value[1] + 10_002) as usize,
        }
    }
}

struct DSU<const SIZE: usize> ([usize; SIZE]);

impl<const SIZE: usize> DSU<SIZE> {
    pub fn new() -> Self {
        Self ([0_usize; SIZE])
    }
    
    pub fn merge_components(&mut self, a: usize, b: usize, connected_count: &mut usize) {
        let rep_a = self.find_representative(a, connected_count);
        let rep_b = self.find_representative(b, connected_count);
        if rep_a != rep_b {
            self.0[rep_b] = rep_a;
            *connected_count -= 1;
        }
    }

    pub fn find_representative(&mut self, element: usize, connected_count: &mut usize) -> usize {
        let el = &mut self.0[element];
        if *el == 0 {
            *el = element;
            *connected_count += 1;
        }
        
        if *el == element {
            element
        } else {
            let e = *el;
            let res = self.find_representative(e, connected_count);
            self.0[element] = res;
            res
        }
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        (n - stones
            .into_iter()
            .map(Coords::from)
            .fold((DSU::<20_003>::new(), 0), |(mut dsu, mut connected_count), c| {
                dsu.merge_components(c.row, c.col, &mut connected_count);
                (dsu, connected_count)
            })
            .1) as i32
    }
}