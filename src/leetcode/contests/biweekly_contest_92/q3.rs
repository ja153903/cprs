#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut score = 0;
        let mut maximum_score = 0;
        let mut result = -1;

        for (i, ch) in customers.char_indices() {
            if ch == 'Y' {
                score += 1;
            } else {
                score -= 1;
            }

            if score > maximum_score {
                maximum_score = score;
                result = i as i32;
            }
        }

        result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        assert_eq!(Solution::best_closing_time(String::from("YYNY")), 2);
        assert_eq!(Solution::best_closing_time(String::from("NNNNN")), 0);
        assert_eq!(Solution::best_closing_time(String::from("YYYY")), 4);
    }
}
