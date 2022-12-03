/// This file contains a template for solving all the Advent of Code problems
/// It has a simple utility to read the input line by line
#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io::{self};

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => Ok(0),
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/<>/data/<>";
    const DATA: &str = "./src/advent_of_code/<>/data/<>";

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 0);
    }
}
