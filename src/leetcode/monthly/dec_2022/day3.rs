#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut frequencies: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            *frequencies.entry(ch).or_insert(0) += 1;
        }

        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort_by(|a, b| {
            let fa = frequencies.get(a).unwrap();
            let fb = frequencies.get(b).unwrap();

            if fa == fb {
                a.cmp(b)
            } else {
                fb.cmp(fa)
            }
        });

        chars.iter().collect::<String>()
    }
}
