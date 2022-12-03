#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io::{self};

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut floor: i32 = 0;

            for line in lines {
                if let Ok(dirs) = line {
                    floor += dirs
                        .chars()
                        .map(|ch| if ch == '(' { 1 } else { -1 })
                        .sum::<i32>();
                }
            }

            Ok(floor)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut floor: i32 = 0;

            for line in lines {
                if let Ok(dirs) = line {
                    for (i, ch) in dirs.char_indices() {
                        floor += match ch {
                            '(' => 1,
                            _ => -1,
                        };

                        if floor < 0 {
                            return Ok(i as i32 + 1);
                        }
                    }
                }
            }

            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2015/data/day1_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2015/data/day1.txt";

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 74);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 1795);
    }
}
