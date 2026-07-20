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

        fn get_lowest_handle(handles: &[Option<usize>; 3]) -> usize {
            debug_assert!(all_handles_found(handles));
            handles.iter().map(|h| h.unwrap()).min().unwrap()
        }

        fn get_highest_handle(handles: &[Option<usize>; 3]) -> usize {
            debug_assert!(all_handles_found(handles));
            handles.iter().map(|h| h.unwrap()).max().unwrap()
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
            res += (get_lowest_handle(&handles) - last_start + 1) * (n - get_highest_handle(&handles));
            last_start = get_lowest_handle(&handles) + 1;
            
            // update lowest handle
            let Some((i, b)) = indeces_iter.next() else {
                break;
            };
            handles[*b].insert(*i);
        }

        res as i32
    }
}