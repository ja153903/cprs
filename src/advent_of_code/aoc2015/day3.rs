#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::HashSet;
use std::io::{self};

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut visited: HashSet<String> = HashSet::new();
            let mut houses = 1;
            let mut x = 0;
            let mut y = 0;

            visited.insert(format!("{},{}", x, y));

            for line in lines {
                if let Ok(dirs) = line {
                    for dir in dirs.chars() {
                        let (nx, ny) = match dir {
                            '>' => (x + 1, y),
                            '^' => (x, y + 1),
                            '<' => (x - 1, y),
                            _ => (x, y - 1),
                        };
                        let hash = format!("{},{}", nx, ny);
                        if !visited.contains(&hash) {
                            visited.insert(hash);
                            houses += 1;
                        }

                        x = nx;
                        y = ny;
                    }
                }
            }

            Ok(houses)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut visited: HashSet<String> = HashSet::new();
            let mut houses = 1;

            let mut x = 0;
            let mut y = 0;

            let mut robo_x = 0;
            let mut robo_y = 0;

            visited.insert(format!("{},{}", x, y));

            for line in lines {
                if let Ok(dirs) = line {
                    for (i, dir) in dirs.char_indices() {
                        let (nx, ny) = if i % 2 == 0 {
                            match dir {
                                '>' => (x + 1, y),
                                '^' => (x, y + 1),
                                '<' => (x - 1, y),
                                _ => (x, y - 1),
                            }
                        } else {
                            match dir {
                                '>' => (robo_x + 1, robo_y),
                                '^' => (robo_x, robo_y + 1),
                                '<' => (robo_x - 1, robo_y),
                                _ => (robo_x, robo_y - 1),
                            }
                        };

                        let hash = format!("{},{}", nx, ny);
                        if !visited.contains(&hash) {
                            visited.insert(hash);
                            houses += 1;
                        }

                        if i % 2 == 0 {
                            x = nx;
                            y = ny;
                        } else {
                            robo_x = nx;
                            robo_y = ny;
                        }
                    }
                }
            }

            Ok(houses)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const DATA: &str = "./src/advent_of_code/aoc2015/data/day3.txt";

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 2572);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 2631);
    }
}
