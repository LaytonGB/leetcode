const CHARS: [u8; 3] = [b'a', b'b', b'c'];

fn byte_to_idx(b: u8) -> usize {
    (b - b'a') as usize
}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.into_bytes();
        let mut char_indeces = vec![];
        
        for (i, b) in s.iter().enumerate() {
            if CHARS.contains(b) {
                char_indeces.push((i, byte_to_idx(*b)));
            }
        }


        let mut indeces_iter = char_indeces.iter();
        let mut handles = [None; 3];

        fn all_handles_found(handles: &[Option<usize>; 3]) -> bool {
            handles.iter().all(|h| h.is_some())
        }

        fn get_handle_bounds(handles: &[Option<usize>; 3]) -> (usize, usize) {
            debug_assert!(all_handles_found(handles));
            handles.iter().fold((usize::MAX, 0), |(min, max), h| {
                let h = h.unwrap();
                (min.min(h), max.max(h))
            })
        }

        for (i, b) in &mut indeces_iter {
            if (handles[*b].replace(*i).is_none() && all_handles_found(&handles)) {
                break;
            }
        }

        if !all_handles_found(&handles) {
            return 0;
        }

        let mut res = 0;
        let mut last_start = 0;
        loop {
            let (h_low, h_high) = get_handle_bounds(&handles);
            res += (h_low - last_start + 1) * (n - h_high);
            last_start = h_low + 1;
            
            // update lowest handle
            let Some((i, b)) = indeces_iter.next() else {
                break;
            };
            handles[*b].insert(*i);
        }

        res as i32
    }
}