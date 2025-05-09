impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let count = s.bytes().fold([0; 73], |mut count, b| {
            count[(b - b'A') as usize] += 1;
            count
        });
        
        count.iter().map(|c| *c / 2 * 2).sum::<i32>()
            + count.iter().any(|c| *c % 2 == 1) as i32
    }
}