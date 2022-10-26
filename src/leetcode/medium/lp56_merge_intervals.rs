#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut result: Vec<Vec<i32>> = Vec::new();
        result.push(intervals[0].to_vec());

        for i in 1..intervals.len() {
            let top = result.last_mut().unwrap();
            let cur = &intervals[i];

            if cur[0] <= top[1] {
                top[1] = cur[1].max(top[1]);
            } else {
                result.push(cur.to_vec());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(intervals);
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];

        assert!(result.iter().all(|num| expected.contains(num)));
    }
}
