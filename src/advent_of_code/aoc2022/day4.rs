#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io;

struct Interval {
    start: i32,
    end: i32,
}

fn to_interval(interval_str: &str) -> Interval {
    let mut parts = interval_str.split("-");

    Interval {
        start: parts.next().unwrap().parse::<i32>().unwrap(),
        end: parts.next().unwrap().parse::<i32>().unwrap(),
    }
}

fn can_full_merge(a: Interval, b: Interval) -> bool {
    (a.start <= b.start && b.end <= a.end) || (b.start <= a.start && a.end <= b.end)
}

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result = 0;

            for line in lines {
                if let Ok(intervals) = line {
                    let mut intervals = intervals.split(",");

                    let first = to_interval(intervals.next().unwrap());
                    let second = to_interval(intervals.next().unwrap());

                    if can_full_merge(first, second) {
                        result += 1;
                    }
                }
            }

            Ok(result)
        }
    }
}

fn can_merge(a: Interval, b: Interval) -> bool {
    if a.start <= b.start {
        b.start <= a.end
    } else {
        a.start <= b.end
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result = 0;

            for line in lines {
                if let Ok(intervals) = line {
                    let mut intervals = intervals.split(",");

                    let first = to_interval(intervals.next().unwrap());
                    let second = to_interval(intervals.next().unwrap());

                    if can_merge(first, second) {
                        result += 1;
                    }
                }
            }

            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day4_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day4.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 459);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 779);
    }
}
