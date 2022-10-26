#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut stack: Vec<(String, i32, i32)> = Vec::new();
        stack.push((String::from("("), 1, 0));

        while !stack.is_empty() {
            if let Some(top) = stack.pop() {
                let (s, open, closed) = top;

                if open == n && closed == n {
                    result.push(s);
                    continue;
                }

                if open < n {
                    stack.push((format!("{}(", s), open + 1, closed));
                }

                if closed < open {
                    stack.push((format!("{})", s), open, closed + 1));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_test_sample_case() {
        let n = 3;
        let parens = vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()"),
        ];

        let mut result = Solution::generate_parenthesis(n);
        result.sort();

        assert_eq!(result, parens);
    }
}
