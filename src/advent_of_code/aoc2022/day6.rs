#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::HashMap;
use std::io;

fn solve(path: &str, window_size: usize) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            for line in lines.flatten() {
                let chars = line.chars().collect::<Vec<char>>();
                let mut map: HashMap<char, usize> = HashMap::new();
                let mut start = 0;

                for end in 0..chars.len() {
                    if let std::collections::hash_map::Entry::Vacant(e) = map.entry(chars[end]) {
                        e.insert(end);

                        if end - start + 1 == window_size {
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

            Ok(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day6_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day6.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = solve(SAMPLE_DATA, 4);
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    pub fn run_part1() {
        let result = solve(DATA, 4);
        assert_eq!(result.unwrap(), 1647);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = solve(SAMPLE_DATA, 14);
        assert_eq!(result.unwrap(), 19);
    }

    #[test]
    pub fn run_part2() {
        let result = solve(DATA, 14);
        assert_eq!(result.unwrap(), 2447);
    }
}
