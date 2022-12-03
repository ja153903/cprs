#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io::{self};

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result: i32 = 0;

            for line in lines {
                if let Ok(statement) = line {
                    let parts = statement
                        .split('x')
                        .map(|ch| ch.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    let l = parts[0];
                    let w = parts[1];
                    let h = parts[2];

                    let min_area = (l * w).min((w * h).min(l * h));

                    result += 2 * l * w + 2 * w * h + 2 * l * h + min_area;
                }
            }

            Ok(result)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result: i32 = 0;

            for line in lines {
                if let Ok(statement) = line {
                    let parts = statement
                        .split('x')
                        .map(|ch| ch.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    let l = parts[0];
                    let w = parts[1];
                    let h = parts[2];

                    let min_perim = 2 * (l + w).min((w + h).min(l + h));

                    result += l * w * h + min_perim;
                }
            }

            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2015/data/day2_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2015/data/day2.txt";

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 1606483);
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
        assert_eq!(result.unwrap(), 3842356);
    }
}
