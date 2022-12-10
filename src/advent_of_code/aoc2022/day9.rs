#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::HashSet;
use std::io;

const DIRS: [[i32; 2]; 4] = [[1, 1], [1, -1], [-1, 1], [-1, -1]];

fn should_jump_diagonally(head: (i32, i32), tail: (i32, i32)) -> bool {
    let is_same_row = head.0 == tail.0;
    let is_same_col = head.1 == tail.1;

    let abs_col_diff = (head.1 - tail.1).abs();
    let abs_row_diff = (head.0 - tail.0).abs();

    let is_4dir_touching = (is_same_row && abs_col_diff == 1) || (is_same_col && abs_row_diff == 1);
    let is_diag_touching = abs_row_diff == 1 && abs_col_diff == 1;

    !is_diag_touching && !is_4dir_touching && !is_same_col && !is_same_row
}

fn solve(path: &str, size: usize) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            let mut knots: Vec<(i32, i32)> = vec![(0, 0); size];

            for line in lines.flatten() {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                let dir = parts[0];
                let mut mv = parts[1]
                    .parse::<i32>()
                    .expect("Something went wrong parsing into i32");

                let (drow, dcol) = match dir {
                    "R" => (0, 1),
                    "L" => (0, -1),
                    "U" => (1, 0),
                    _ => (-1, 0),
                };

                while mv > 0 {
                    knots[0].0 += drow;
                    knots[0].1 += dcol;

                    for i in 1..knots.len() {
                        let prev = knots[i - 1];
                        let curr = knots[i];

                        let abs_row_diff = (prev.0 - curr.0).abs();
                        let abs_col_diff = (prev.1 - curr.1).abs();

                        let should_move_normally = (abs_row_diff > 1 && abs_col_diff == 0)
                            || (abs_col_diff > 1 && abs_row_diff == 0);

                        if should_jump_diagonally(prev, curr) {
                            // if it goes diagonally, we just choose to be 1 off
                            if abs_col_diff == 2 {
                                if curr.1 > prev.1 {
                                    if curr.0 > prev.0 {
                                        knots[i] = (curr.0 - 1, curr.1 - 1);
                                    } else {
                                        knots[i] = (curr.0 + 1, curr.1 - 1);
                                    }
                                } else if curr.0 > prev.0 {
                                    knots[i] = (curr.0 - 1, curr.1 + 1);
                                } else {
                                    knots[i] = (curr.0 + 1, curr.1 + 1);
                                }
                            }

                            if abs_row_diff == 2 {
                                if curr.0 > prev.0 {
                                    if curr.1 > prev.1 {
                                        knots[i] = (curr.0 - 1, curr.1 - 1);
                                    } else {
                                        knots[i] = (curr.0 - 1, curr.1 + 1);
                                    }
                                } else if curr.1 > prev.1 {
                                    knots[i] = (curr.0 + 1, curr.1 - 1);
                                } else {
                                    knots[i] = (curr.0 + 1, curr.1 + 1);
                                }
                            }
                        } else if should_move_normally {
                            // we have to move in the direction of the head
                            if abs_row_diff == 0 {
                                // same row, so we need column to be 1 apart
                                if curr.1 > prev.1 {
                                    knots[i] = (curr.0, curr.1 - 1);
                                } else {
                                    knots[i] = (curr.0, curr.1 + 1);
                                }
                            }

                            if abs_col_diff == 0 {
                                if curr.0 > prev.0 {
                                    knots[i] = (curr.0 - 1, curr.1);
                                } else {
                                    knots[i] = (curr.0 + 1, curr.1);
                                }
                            }
                        }
                    }

                    let last = knots.last().unwrap();
                    visited.insert((last.0, last.1));

                    mv -= 1;
                }
            }

            Ok(visited.len() as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day9_sample.txt";
    const SAMPLE_DIFFICULT_DATA: &str =
        "./src/advent_of_code/aoc2022/data/day9_difficult_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day9.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = solve(SAMPLE_DATA, 2);
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    pub fn run_part1() {
        let result = solve(DATA, 2);
        assert_eq!(result.unwrap(), 5735);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = solve(SAMPLE_DATA, 10);
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    pub fn run_part2_sample_difficult() {
        let result = solve(SAMPLE_DIFFICULT_DATA, 10);
        assert_eq!(result.unwrap(), 36);
    }

    #[test]
    pub fn run_part2() {
        let result = solve(DATA, 10);
        assert_eq!(result.unwrap(), 2478);
    }
}
