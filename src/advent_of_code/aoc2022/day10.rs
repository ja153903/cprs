#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut register = 1;
            let mut ranges: Vec<i32> = vec![];
            let cycles: HashSet<usize> =
                HashSet::from_iter(vec![20, 60, 100, 140, 180, 220].iter().cloned());
            let mut total_signal_strength = 0;

            // signal strength = cycle number * register value
            // keep a hash map of the signal strengths

            for line in lines.flatten() {
                let mut command = line.split_whitespace();
                if let Some(cmd) = command.next() {
                    if cmd == "addx" {
                        if let Some(value) = command.next() {
                            let value = value
                                .parse::<i32>()
                                .expect("Something went wrong parsing the value");

                            ranges.push(register);
                            if cycles.contains(&ranges.len()) {
                                total_signal_strength += ranges.len() as i32 * register;
                            }

                            ranges.push(register + value);
                            if cycles.contains(&ranges.len()) {
                                total_signal_strength += ranges.len() as i32 * register;
                            }

                            register += value;
                        }
                    } else {
                        // all we do here is set the current register value for this entry
                        ranges.push(register);
                        if cycles.contains(&ranges.len()) {
                            total_signal_strength += ranges.len() as i32 * register;
                        }
                    }
                }
            }

            Ok(total_signal_strength)
        }
    }
}

fn get_lit(cycle: i32, sprite_pos: i32) -> char {
    let mod40 = cycle % 40;

    if mod40 < sprite_pos || mod40 > sprite_pos + 2 {
        '.'
    } else {
        '#'
    }
}

fn part2(path: &str) -> io::Result<String> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut cycle: i32 = 0;
            let mut board: Vec<char> = vec![];
            let mut sprite_pos = 1;

            for line in lines.flatten() {
                let mut command = line.split_whitespace();
                if let Some(cmd) = command.next() {
                    if cmd == "addx" {
                        if let Some(value) = command.next() {
                            let value = value
                                .parse::<i32>()
                                .expect("Something went wrong parsing the value");

                            cycle += 1;
                            board.push(get_lit(cycle, sprite_pos));

                            cycle += 1;
                            board.push(get_lit(cycle, sprite_pos));

                            sprite_pos += value;
                        }
                    } else {
                        cycle += 1;
                        board.push(get_lit(cycle, sprite_pos));
                    }
                }
            }

            // for (i, ch) in board.iter().enumerate() {
            //     if i > 0 && i % 40 == 0 {
            //         println!();
            //     }
            //     print!("{} ", ch);
            // }

            Ok(String::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day10_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day10.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 13140);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 11820);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), String::new());
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), String::new());
    }
}
