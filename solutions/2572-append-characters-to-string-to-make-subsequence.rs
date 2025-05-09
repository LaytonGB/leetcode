impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut t_it = t.chars();
        let mut tco = t_it.next();

        for sc in s.chars() {
            if let Some(tc) = tco {
                if sc == tc {
                    tco = t_it.next();
                }
            } else {
                return 0;
            }
        }

        t_it.count() as i32 + tco.is_some() as i32
    }
}