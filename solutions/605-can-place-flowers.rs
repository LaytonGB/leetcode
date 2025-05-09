impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if n == 0 {
                return true;
            }
            if flowerbed[i] == 0 {
                let left = i == 0 || flowerbed[i - 1] == 0;
                let right = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;
                if left && right {
                    n -= 1;
                    flowerbed[i] = 1;
                }
            }
        }
        n == 0
    }
}