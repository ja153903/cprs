#![allow(dead_code)]

use std::collections::HashSet;
use std::io::{self};

use itertools::Itertools;

use crate::advent_of_code::helpers::file::read_lines;

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result = 0;

            for line in lines {
                if let Ok(rucksack) = line {
                    let mut intersection: HashSet<char> = HashSet::new();
                    let mid = rucksack.len() / 2;
                    let (left, right) = rucksack.split_at(mid);

                    for ch in left.chars() {
                        intersection.insert(ch);
                    }

                    for ch in right.chars() {
                        if intersection.contains(&ch) {
                            if ch.is_uppercase() {
                                result += (ch as u32) as i32 - 64 + 26;
                            } else {
                                result += (ch as u32) as i32 - 96;
                            }
                            break;
                        }
                    }
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
            let mut result = 0;

            for lines in &lines.chunks(3) {
                let ch = lines
                    .map(|item| item.unwrap())
                    .reduce(|accum, item| get_intersection(accum, item))
                    .unwrap()
                    .chars()
                    .next()
                    .expect("Could not find an intersection");

                if ch.is_uppercase() {
                    result += (ch as u32) as i32 - 64 + 26;
                } else {
                    result += (ch as u32) as i32 - 96;
                }
            }

            Ok(result)
        }
    }
}

fn get_intersection(s: String, t: String) -> String {
    if s.is_empty() {
        return t;
    }

    if t.is_empty() {
        return s;
    }

    let mut uniq_s: HashSet<char> = HashSet::new();
    let mut uniq_t: HashSet<char> = HashSet::new();

    for ch in s.chars() {
        uniq_s.insert(ch);
    }

    for ch in t.chars() {
        uniq_t.insert(ch);
    }

    uniq_s.intersection(&uniq_t).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day3_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day3.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 157);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 8072);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 70);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 2567);
    }
}
