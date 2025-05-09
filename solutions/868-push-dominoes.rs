impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        unsafe {
            let mut s = dominoes.as_mut_vec();
            let mut edited = true;
            while edited {
                edited = false;

                let mut next = s.to_vec();
                for i in 0..s.len() {
                    let c = s[i];

                    if c != b'.' {
                        continue;
                    }

                    match (s.get(i - 1), s.get(i + 1)) {
                        (Some(b'R'), Some(b'L')) => {}
                        (Some(b'R'), _) => {
                            next[i] = b'R';
                            edited = true;
                        }
                        (_, Some(b'L')) => {
                            next[i] = b'L';
                            edited = true;
                        }
                        _ => {}
                    }
                }
                *s = next;
            }
        }

        dominoes
    }
}