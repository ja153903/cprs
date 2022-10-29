#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        // this is a stack problem, but how we navigate the stack
        // There are 4 types of characters:
        // 1. If the item is a number, then we should keep incrementing our current count
        // 2. If the item is an alphabetic character, then we should keep adding the characters to
        //    some buffer
        // 3. If the item is an opening bracket, then we store the current count and the current
        //    character to some stack
        // 4. If the item is a closing bracket, then the
        let mut count: i32 = 0;
        let mut current: String = String::new();

        let mut count_stack: Vec<i32> = Vec::new();
        let mut current_stack: Vec<String> = Vec::new();

        for ch in s.chars() {
            if ch.is_numeric() {
                let as_i32: i32 = ch.to_digit(10).unwrap() as i32;
                count = count * 10 + as_i32;
            } else if ch.is_alphabetic() {
                current.push(ch);
            } else if ch == '[' {
                count_stack.push(count);
                current_stack.push(current);

                count = 0;
                current = String::new();
            } else {
                let top_count = count_stack.pop().unwrap();
                let top_current = current_stack.pop().unwrap();

                let mut to_append = String::new();

                for _ in 0..top_count {
                    to_append.push_str(&current);
                }

                current = format!("{}{}", top_current, to_append);
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let s = String::from("3[a]2[bc]");
        let expected = String::from("aaabcbc");

        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    pub fn should_pass_another_sample_case() {
        let s = String::from("3[a2[c]]");
        let expected = String::from("accaccacc");

        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    pub fn should_pass_the_last_sample_case() {
        let s = String::from("2[abc]3[cd]ef");
        let expected = String::from("abcabccdcdcdef");

        assert_eq!(Solution::decode_string(s), expected);
    }
}
