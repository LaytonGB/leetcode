impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();
        
        let mut res = vec![folder[0].clone()];
        for f in folder.into_iter().skip(1) {
            let path_str = res.last().unwrap().clone() + "/";
            if !f.starts_with(&path_str) {
                res.push(f);
            }
        }

        res
    }
}