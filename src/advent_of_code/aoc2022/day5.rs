#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::{collections::HashMap, io};

fn build_sample_stack_map() -> HashMap<char, Vec<char>> {
    let mut result: HashMap<char, Vec<char>> = HashMap::new();

    result.insert('1', vec!['Z', 'N']);
    result.insert('2', vec!['M', 'C', 'D']);
    result.insert('3', vec!['P']);

    result
}

fn build_stack_map() -> HashMap<char, Vec<char>> {
    let mut result: HashMap<char, Vec<char>> = HashMap::new();

    result.insert('1', vec!['H', 'T', 'Z', 'D']);
    result.insert('2', vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S']);
    result.insert('3', vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H']);
    result.insert('4', vec!['L', 'C', 'N', 'F', 'H', 'Z']);
    result.insert('5', vec!['G', 'L', 'F', 'Q', 'S']);
    result.insert('6', vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S']);
    result.insert('7', vec!['Z', 'F', 'J']);
    result.insert('8', vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q']);
    result.insert('9', vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D']);

    result
}

fn part1(path: &str, is_sample: bool) -> io::Result<String> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut stack_map = if is_sample {
                build_sample_stack_map()
            } else {
                build_stack_map()
            };

            let keys = if is_sample {
                vec!['1', '2', '3']
            } else {
                vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
            };

            for line in lines {
                if let Ok(direction) = line {
                    let parts = direction.split_ascii_whitespace().collect::<Vec<&str>>();
                    let mut count = parts[1].parse::<i32>().unwrap();
                    let from_hash = parts[3].chars().next().unwrap();
                    let to_hash = parts[5].chars().next().unwrap();

                    while count > 0 {
                        if let Some(v) = stack_map.get_mut(&from_hash) {
                            if let Some(c) = v.pop() {
                                stack_map.entry(to_hash).and_modify(|v| v.push(c));
                            }
                        }

                        count -= 1;
                    }
                }
            }

            let mut result = String::new();

            for key in keys.iter() {
                if let Some(v) = stack_map.get(key) {
                    if let Some(&c) = v.last() {
                        result.push(c);
                    }
                }
            }

            Ok(result)
        }
    }
}

fn part2(path: &str, is_sample: bool) -> io::Result<String> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut stack_map = if is_sample {
                build_sample_stack_map()
            } else {
                build_stack_map()
            };

            let keys = if is_sample {
                vec!['1', '2', '3']
            } else {
                vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
            };

            for line in lines {
                if let Ok(direction) = line {
                    let parts = direction.split_ascii_whitespace().collect::<Vec<&str>>();
                    let mut count = parts[1].parse::<i32>().unwrap();
                    let from_hash = parts[3].chars().next().unwrap();
                    let to_hash = parts[5].chars().next().unwrap();

                    let mut temp_buffer: Vec<char> = Vec::new();

                    while count > 0 {
                        if let Some(v) = stack_map.get_mut(&from_hash) {
                            if let Some(c) = v.pop() {
                                temp_buffer.push(c);
                            }
                        }

                        count -= 1;
                    }

                    while !temp_buffer.is_empty() {
                        stack_map
                            .entry(to_hash)
                            .and_modify(|v| v.push(temp_buffer.pop().unwrap()));
                    }
                }
            }

            let mut result = String::new();

            for key in keys.iter() {
                if let Some(v) = stack_map.get(key) {
                    if let Some(&c) = v.last() {
                        result.push(c);
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

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day5_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day5.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA, true);
        assert_eq!(result.unwrap(), String::from("CMZ"));
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA, false);
        assert_eq!(result.unwrap(), String::from("RFFFWBPNS"));
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA, true);
        assert_eq!(result.unwrap(), String::from("MCD"));
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA, false);
        assert_eq!(result.unwrap(), String::from("CQQBBJFCS"));
    }
}
