const NONE: Option<(String, i32)> = None;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut c: [Option<(String, i32)>; 101] = [NONE; 101];
        let mut i = 0;
        for p in paths {
            // take from p
            let mut p = p.into_iter();
            let p1 = p.next().unwrap();
            let p2 = p.next().unwrap();

            // find and alter existing count or create one
            if let Some(c) = c[..i].iter_mut().find(|c| if let Some(ref c) = c {c.0 == p1} else {false}) {
                let c = c.as_mut().unwrap();
                c.1 -= 1;
            } else {
                c[i] = Some((p1, -1));
                i += 1;
            }
            if let Some(c) = c[..i].iter_mut().find(|c| if let Some(ref c) = c {c.0 == p2} else {false}) {
                let c = c.as_mut().unwrap();
                c.1 += 1;
            } else {
                c[i] = Some((p2, 1));
                i += 1;
            }
        }
        c.iter().find(|c| c.as_ref().unwrap().1 == 1).unwrap().as_ref().unwrap().0.to_owned()
    }
}