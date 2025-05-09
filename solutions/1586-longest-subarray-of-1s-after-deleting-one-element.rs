impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (subarray_lengths, first_one, last_one, _) = nums.iter()
            .enumerate()
            .fold((Vec::new(), None, None, 0), |(mut v, first_one, last_one, last_n), (i, &n)| {
                match (first_one, last_one, n, last_n, *v.last().unwrap_or(&0)) {
                    (_, _, 0, 0, 1..) => {
                        v.push(0);
                        (v, first_one, last_one, 0)
                    }
                    (_, _, 0, ..) => (v, first_one, last_one, 0),
                    (None, _, 1, ..) => {
                        v.push(1);
                        (v, Some(i), Some(i), 1)
                    }
                    (_, Some(j), 1, ..) => {
                        if j < i - 1 {
                            v.push(1);
                        } else {
                            *v.last_mut().unwrap() += 1;
                        }
                        (v, first_one, Some(i), 1)
                    }
                    _ => unreachable!(),
                }
            });
        // println!("{:?}", subarray_lengths);
        subarray_lengths.windows(2)
            .map(|w| w[0] + w[1])
            .max()
            .unwrap_or(
                // there is only 1 group of 1s
                match (nums.len(), first_one, last_one) {
                    (_, None, ..) => 0,
                    (len, Some(0), Some(last)) if len - 1 == last => len as i32 - 1,
                    (len, _, _) => subarray_lengths[0]
                }
            )
    }
}