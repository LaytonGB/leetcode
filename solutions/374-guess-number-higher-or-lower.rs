/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut l, mut h) = (0, n);
        let mut m = h / 2;
        while l < h {
            m = l + (h - l) / 2;
            match guess(m + 1) {
                1 => l = m + 1,
                -1 => h = m,
                _ => break,
            }
        }
        m + 1
    }
}