#![allow(dead_code)]

struct Solution;

impl Solution {
    /// You are given a string s and an integer k, a k duplicate
    /// removal consists of choosing k adjacent and equal letters from s and removing them,
    /// causing the left and the right side of the deleted substring to concatenate together.
    /// We repeatedly make k duplicate removals on s until we no longer can.
    /// Return the final string after all such duplicate removals have been made.
    /// It is guaranteed that the answer is unique.
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, i32)> = Vec::new();

        for ch in s.chars() {
            if let Some(top) = stack.last() {
                let (t_ch, t_count) = top;

                if *t_ch == ch {
                    stack.push((ch, t_count + 1));
                } else {
                    stack.push((ch, 1));
                }
            } else {
                stack.push((ch, 1));
            }

            let should_remove_k_elems = if let Some(top) = stack.last() {
                let (_t_ch, t_count) = top;
                *t_count == k
            } else {
                k > 0
            };

            if should_remove_k_elems {
                for _ in 0..k {
                    stack.pop();
                }
            }
        }

        return stack.iter().map(|(ch, _count)| ch).collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_cases() {
        let mut s = String::from("abcd");
        let mut k = 2;

        assert_eq!(Solution::remove_duplicates(s, k), String::from("abcd"));

        s = String::from("deeedbbcccbdaa");
        k = 3;

        assert_eq!(Solution::remove_duplicates(s, k), String::from("aa"));

        s = String::from("pbbcggttciiippooaais");
        k = 2;

        assert_eq!(Solution::remove_duplicates(s, k), String::from("ps"));
    }
}
