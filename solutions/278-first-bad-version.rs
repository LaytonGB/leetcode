// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let (mut l, mut h) = (0, (n + 1) as usize);
        let mut m = h / 2;
        while l < h {
            if self.isBadVersion(m as i32) {
                h = m;
            } else {
                l = m + 1;
            }
            m = l + ((h - l) / 2);
        }
        m as i32
    }
}