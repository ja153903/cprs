#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::HashMap;
use std::io;

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            for line in lines {
                if let Ok(input) = line {
                    let chars = input.chars().collect::<Vec<char>>();
                    let mut map: HashMap<char, usize> = HashMap::new();
                    let mut start = 0;

                    for end in 0..chars.len() {
                        if !map.contains_key(&chars[end]) {
                            map.insert(chars[end], end);

                            if end - start + 1 == 4 {
                                return Ok(end as i32 + 1);
                            }
                        } else {
                            // update starting point until we get to chars[start] == chars[end]
                            while start < end && chars[start] != chars[end] {
                                map.remove(&chars[start]);
                                start += 1;
                            }

                            start += 1;
                        }
                    }
                }
            }

            Ok(1)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            for line in lines {
                if let Ok(input) = line {
                    let chars = input.chars().collect::<Vec<char>>();
                    let mut map: HashMap<char, usize> = HashMap::new();
                    let mut start = 0;

                    for end in 0..chars.len() {
                        if !map.contains_key(&chars[end]) {
                            map.insert(chars[end], end);

                            if end - start + 1 == 14 {
                                return Ok(end as i32 + 1);
                            }
                        } else {
                            // update starting point until we get to chars[start] == chars[end]
                            while start < end && chars[start] != chars[end] {
                                map.remove(&chars[start]);
                                start += 1;
                            }

                            start += 1;
                        }
                    }
                }
            }

            Ok(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day6_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day6.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 1647);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 19);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 2447);
    }
}
