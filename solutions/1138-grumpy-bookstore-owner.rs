impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let n = customers.len();
        if n == 0 {
            return 0;
        }

        let m = minutes as usize;
        if n <= m {
            return customers.iter().sum();
        }

        let zipped_iter = customers.iter()
            .zip(grumpy.iter());
        
        let always_satisfied = zipped_iter.clone()
            .filter(|(_, g)| **g == 0)
            .map(|(c, _)| *c)
            .sum::<i32>();

        let mut rolling_bonus = zipped_iter.take(m)
            .filter(|(_, g)| **g == 1)
            .map(|(c, _)| *c)
            .sum::<i32>();
        let mut max_bonus = rolling_bonus;
        for i in m..n {
            if grumpy[i - m] == 1 {
                rolling_bonus -= customers[i - m];
            }
            if grumpy[i] == 1 {
                rolling_bonus += customers[i];
                max_bonus = max_bonus.max(rolling_bonus);
            }
        }

        always_satisfied + max_bonus
    }
}