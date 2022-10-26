#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        // we can think of this problem like two sum
        // the target in this case should be 60
        // for every time, we should check if 60 - (time[i] % 60) is in the map
        let mut result = 0;
        let mut seen: HashMap<i32, i32> = HashMap::new();

        for t in time.iter() {
            let with_mod = t % 60;

            // NOTE: To handle an edge case where the time is 0, we add an extra mod here.
            if let Some(count) = seen.get(&((60 - with_mod) % 60)) {
                result += count;
            }

            *seen.entry(with_mod).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let time = vec![30, 20, 150, 100, 40];

        let result = Solution::num_pairs_divisible_by60(time);
        let expected = 3;

        assert_eq!(result, expected);
    }
}
