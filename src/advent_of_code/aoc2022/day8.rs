#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::io;

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut grid: Vec<Vec<i32>> = Vec::new();

            for line in lines.flatten() {
                let mut current: Vec<i32> = Vec::with_capacity(line.len());
                for ch in line.chars() {
                    if let Some(num) = ch.to_digit(10) {
                        current.push(num as i32);
                    }
                }

                grid.push(current);
            }

            let mut num_trees = 2 * (grid.len() + grid[0].len()) as i32 - 4;

            for i in 1..grid.len() - 1 {
                for j in 1..grid[0].len() - 1 {
                    let anchor = grid[i][j];

                    let west_anchor = grid[i][0];
                    if west_anchor < anchor {
                        let mut should_add_tree = true;
                        for col in 1..j {
                            if grid[i][col] >= anchor {
                                should_add_tree = false;
                                break;
                            }
                        }

                        if should_add_tree {
                            num_trees += 1;
                            continue;
                        }
                    }

                    let east_anchor = grid[i][grid[0].len() - 1];
                    if east_anchor < anchor {
                        let mut should_add_tree = true;
                        for col in (j + 1)..grid[0].len() {
                            if grid[i][col] >= anchor {
                                should_add_tree = false;
                                break;
                            }
                        }

                        if should_add_tree {
                            num_trees += 1;
                            continue;
                        }
                    }

                    let north_anchor = grid[0][j];
                    if north_anchor < anchor {
                        let mut should_add_tree = true;
                        for row in 1..i {
                            if grid[row][j] >= anchor {
                                should_add_tree = false;
                                break;
                            }
                        }

                        if should_add_tree {
                            num_trees += 1;
                            continue;
                        }
                    }

                    let south_anchor = grid[grid.len() - 1][j];
                    if south_anchor < anchor {
                        let mut should_add_tree = true;
                        for row in (i + 1)..grid.len() {
                            if grid[row][j] >= anchor {
                                should_add_tree = false;
                                break;
                            }
                        }

                        if should_add_tree {
                            num_trees += 1;
                            continue;
                        }
                    }
                }
            }

            Ok(num_trees)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut grid: Vec<Vec<i32>> = Vec::new();

            for line in lines.flatten() {
                let mut current: Vec<i32> = Vec::with_capacity(line.len());
                for ch in line.chars() {
                    if let Some(num) = ch.to_digit(10) {
                        current.push(num as i32);
                    }
                }

                grid.push(current);
            }

            let mut max_scenic_score = 0;

            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    let anchor = grid[i][j];

                    let mut row = i as i32;
                    let mut north_trees = 0;

                    while row >= 0 {
                        row -= 1;
                        if row < 0 {
                            break;
                        }
                        if grid[row as usize][j] >= anchor {
                            north_trees += 1;
                            break;
                        }

                        north_trees += 1;
                    }

                    let mut row = i as i32;
                    let mut south_trees = 0;

                    while row < grid.len() as i32 {
                        row += 1;
                        if row >= grid.len() as i32 {
                            break;
                        }
                        if grid[row as usize][j] >= anchor {
                            south_trees += 1;
                            break;
                        }

                        south_trees += 1;
                    }

                    let mut col = j as i32;
                    let mut east_trees = 0;

                    while col < grid[0].len() as i32 {
                        col += 1;
                        if col >= grid[0].len() as i32 {
                            break;
                        }
                        if grid[i][col as usize] >= anchor {
                            east_trees += 1;
                            break;
                        }
                        east_trees += 1;
                    }

                    let mut col = j as i32;
                    let mut west_trees = 0;

                    while col >= 0 {
                        col -= 1;
                        if col < 0 {
                            break;
                        }
                        if grid[i][col as usize] >= anchor {
                            west_trees += 1;
                            break;
                        }

                        west_trees += 1;
                    }

                    max_scenic_score =
                        max_scenic_score.max(north_trees * south_trees * west_trees * east_trees);
                }
            }

            Ok(max_scenic_score)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day8_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day8.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 1832);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 157320);
    }
}
