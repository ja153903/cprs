#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 1 {
            1
        } else {
            let mut res = (1, 1);

            for _ in 0..n {
                res = (res.1, res.0 + res.1);
            }

            res.0
        }
    }
}
