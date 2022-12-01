#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut char_pos: Vec<usize> = vec![0; 26];
        let mut result = 0;

        for (i, ch) in keyboard.char_indices() {
            let char_code = (ch as u32) - 97;
            char_pos[char_code as usize] = i;
        }

        let mut prev = 0;

        for ch in word.chars() {
            let char_code = (ch as u32) - 97;
            let current = char_pos[char_code as usize] as i32;
            result += (current - prev).abs();
            prev = current;
        }

        result
    }
}
