#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, Lines};

fn build_directory(lines: Lines<BufReader<File>>) -> HashMap<String, i64> {
    let mut directory: HashMap<String, i64> = HashMap::new();
    let mut history: Vec<String> = vec![];

    for line in lines.flatten() {
        if line.starts_with("$ cd") {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if let Some(&last) = parts.last() {
                if last == ".." {
                    history.pop();
                } else {
                    history.push(last.to_string());
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let size = parts[0]
                .parse::<i64>()
                .expect("Something went wrong parsing");

            let mut cum_dir = String::new();
            for dir in history.iter() {
                cum_dir.push_str(dir);
                directory
                    .entry(cum_dir.clone())
                    .and_modify(|node| {
                        *node += size;
                    })
                    .or_insert(size);
            }
        }
    }

    directory
}

fn part1(path: &str) -> io::Result<i64> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let directory = build_directory(lines);

            let mut result: i64 = 0;

            for &value in directory.values() {
                if value <= 100000 {
                    result += value;
                }
            }

            Ok(result)
        }
    }
}

fn part2(path: &str) -> io::Result<i64> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut directory = build_directory(lines);
            let total_disk_space: i64 = 70000000;
            let free_space_needed: i64 = 30000000;
            let current_total_space_used = directory.get("/").unwrap();

            let current_unused_disk_space = total_disk_space - current_total_space_used;

            directory.remove("/");

            let disk_space = directory
                .values()
                .sorted_by_key(|&x| x)
                .copied()
                .collect::<Vec<i64>>();

            for &space in disk_space.iter() {
                if current_unused_disk_space + space >= free_space_needed {
                    return Ok(space);
                }
            }

            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day7_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day7.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 95437);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 2061777);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 24933642);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 4473403);
    }
}
