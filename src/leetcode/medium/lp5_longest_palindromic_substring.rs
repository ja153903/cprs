#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_length: i32 = 0;
        let mut max_start: usize = 0;

        for i in 0..s.len() {
            let (even_length, even_start) = Solution::extend_palindrome(&s, i, i + 1);
            let (odd_length, odd_start) = Solution::extend_palindrome(&s, i, i);

            if even_length > max_length {
                max_length = even_length;
                max_start = even_start;
            }

            if odd_length > max_length {
                max_length = odd_length;
                max_start = odd_start;
            }
        }

        s[max_start..max_start + (max_length as usize)].to_string()
    }

    pub fn extend_palindrome(s: &String, left: usize, right: usize) -> (i32, usize) {
        let mut max_length = 0;
        let mut max_start = 0;

        let mut left = left as i32;
        let mut right = right as i32;

        while left >= 0
            && right < s.len() as i32
            && s[(left as usize)..(left as usize) + 1] == s[(right as usize)..(right as usize) + 1]
        {
            max_length = right - left + 1;
            max_start = left as usize;
            left -= 1;
            right += 1;
        }

        (max_length, max_start)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    pub fn should_pass_sample_case() {
        let s = String::from("babad");

        let valid_solutions: HashSet<String> = HashSet::from_iter(
            vec![String::from("bab"), String::from("aba")]
                .iter()
                .cloned(),
        );

        let result = Solution::longest_palindrome(s);

        assert!(valid_solutions.contains(&result));
    }
}
