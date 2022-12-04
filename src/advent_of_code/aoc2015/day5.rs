#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io;

fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
}

fn is_banned(a: char, b: char) -> bool {
    match (a, b) {
        ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
        _ => false,
    }
}

fn is_nice(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut num_vowels = 0;
    let mut num_pairs = 0;

    for i in 0..chars.len() {
        if is_vowel(chars[i]) {
            num_vowels += 1;
        }

        if i > 0 && chars[i - 1] == chars[i] {
            num_pairs += 1;
        }

        if i > 0 && is_banned(chars[i - 1], chars[i]) {
            return false;
        }
    }

    num_vowels >= 3 && num_pairs >= 1
}

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut ls = 0;

            for line in lines {
                if let Ok(s) = line {
                    if is_nice(s) {
                        ls += 1;
                    }
                }
            }

            Ok(ls)
        }
    }
}

fn is_new_nice(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut num_palindromes = 0;
    let mut num_pairs = 0;

    for i in 1..chars.len() - 1 {
        if chars[i - 1] == chars[i + 1] {
            num_palindromes += 1;
        }
    }

    for i in 1..chars.len() {
        for j in (i + 2)..chars.len() {
            if chars[i - 1] == chars[j - 1] && chars[i] == chars[j] {
                num_pairs += 1;
            }
        }
    }

    num_palindromes >= 1 && num_pairs >= 1
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut ls = 0;

            for line in lines {
                if let Ok(s) = line {
                    if is_new_nice(s) {
                        ls += 1;
                    }
                }
            }

            Ok(ls)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const DATA: &str = "./src/advent_of_code/aoc2015/data/day5.txt";

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 238);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 69);
    }
}
