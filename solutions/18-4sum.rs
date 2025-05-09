use std::cmp::Ordering;

impl Solution {
    pub fn four_sum(nums:Vec<i32>, target:i32) -> Vec<Vec<i32>> {
		Self::k_sum_init(nums, target, 4)
    }

    fn k_sum_init(mut a:Vec<i32>, target:i32, k:i32) -> Vec<Vec<i32>> {
        if k < 2 || a.len() < k as usize { return Vec::new(); }
        a.sort();
        let mut res = Vec::new();
        Self::k_sum(&a, target, k, 0, &mut res, &mut Vec::new());
        res
    }

    fn k_sum(a:&Vec<i32>, target:i32, k:i32, start:usize, res:&mut Vec<Vec<i32>>, path:&mut Vec<i32>) {
        if a[start].checked_mul(k).unwrap_or(i32::MIN) > target
        || a.last().unwrap().checked_mul(k).unwrap_or(i32::MAX) < target {
            return;
        }
        
        if k == 2 { // 2 Sum
            let mut left = start;
            let mut right = a.len() - 1;
            while left < right {
                let sum = a[left].checked_add(a[right]);
                if sum.is_none() { continue; }
                match sum.unwrap().cmp(&target) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        res.push(path.clone());
                        let mut last = res.last_mut().unwrap();
                        [a[left], a[right]].into_iter().for_each(|&n| last.push(n));
                        left += 1;
                        right -= 1;
                        while left < right && a[left] == a[left - 1] { left += 1; }
                        while left < right && a[right] == a[right + 1] { right -= 1 ;}
                    }
                }
            }
        } else { // k Sum
            for i in start..=a.len()-k as usize {
                let a_mul_k = a[i].checked_mul(k);
                if a_mul_k.is_some() && a_mul_k.unwrap() > target { break; }
                if i > start && a[i] == a[i - 1] { continue; }
                let too_small = a.last().unwrap().checked_mul(k - 1).and_then(|n| n.checked_add(a[i]));
                if too_small.is_some() && too_small.unwrap() < target { continue; }
                let k_mul = a[i].checked_mul(k);
                if k_mul.is_some() && k_mul.unwrap() == target {
                    if a[i] == a[i - 1 + k as usize] {
                        res.push(path.clone());
                        res.last_mut().unwrap().append(&mut vec![a[i]; k as usize]);
                    }
                    break;
                }
                let new_target = target.checked_sub(a[i]);
                if new_target.is_some() {
                    path.push(a[i]);
                    Self::k_sum(&a, new_target.unwrap(), k - 1, i + 1, res, path);
                    path.pop(); // backtracking
                }
            }
        }
    }
}