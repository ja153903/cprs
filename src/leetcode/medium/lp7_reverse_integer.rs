#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;

        let mut abs = x.abs();
        let mut result: i32 = 0;

        while abs > 0 {
            let (current, is_overflow) = result.overflowing_mul(10);

            if is_overflow {
                return 0;
            }

            let (current, is_overflow) = current.overflowing_add(abs % 10);

            if is_overflow {
                return 0;
            }

            abs = abs / 10;
            result = current;
        }

        if is_negative {
            -result
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
