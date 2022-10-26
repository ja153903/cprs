#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut start_times: Vec<i32> = vec![];
        let mut end_times: Vec<i32> = vec![];

        for interval in intervals.iter() {
            start_times.push(interval[0]);
            end_times.push(interval[1]);
        }

        start_times.sort();
        end_times.sort();

        let mut result: i32 = 0;

        let mut start_index: usize = 0;
        let mut end_index: usize = 0;

        while start_index < start_times.len() {
            if start_times[start_index] < end_times[end_index] {
                result += 1;
            } else {
                end_index += 1;
            }

            start_index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        let expected = 2;

        assert_eq!(Solution::min_meeting_rooms(intervals), expected);
    }
}
