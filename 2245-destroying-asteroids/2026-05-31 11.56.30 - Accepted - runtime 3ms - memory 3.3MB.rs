impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as usize;
        asteroids.sort_unstable();
        for a in asteroids.into_iter().map(|a| a as usize) {
            if mass < a {
                return false;
            }
            mass += a;
        }
        true
    }
}