use std::io::{self};

use crate::advent_of_code::helpers::file::read_lines;

fn part1(path: &str) -> io::Result<()> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => Ok(()),
    }
}

fn part2(path: &str) -> io::Result<()> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day3_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day3.txt";

    #[ignore = "no impl"]
    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
    }

    #[ignore = "no impl"]
    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
    }

    #[ignore = "no impl"]
    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
    }

    #[ignore = "no impl"]
    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
    }
}
