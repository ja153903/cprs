#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

fn part1(path: &str) -> io::Result<i32> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let mut max_calorie: i32 = 0;
    let mut current_calorie: i32 = 0;

    for line in lines {
        if let Ok(calorie) = line {
            if calorie.is_empty() {
                // reset calorie count
                current_calorie = 0;
            } else {
                current_calorie += calorie.parse::<i32>().unwrap();
            }

            max_calorie = max_calorie.max(current_calorie);
        }
    }

    Ok(max_calorie)
}

fn part2(path: &str) -> io::Result<i32> {
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut current_calorie: i32 = 0;

    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(calorie) = line {
            if calorie.is_empty() {
                min_heap.push(Reverse(current_calorie));
                if min_heap.len() > 3 {
                    min_heap.pop();
                }
                current_calorie = 0;
            } else {
                current_calorie += calorie.parse::<i32>().unwrap();
            }
        }
    }

    min_heap.push(Reverse(current_calorie));
    if min_heap.len() > 3 {
        min_heap.pop();
    }

    Ok(min_heap.iter().map(|&rev| rev.0).sum())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day1_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day1.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        if let Ok(res) = result {
            assert_eq!(res, 24000);
        }
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        if let Ok(res) = result {
            assert_eq!(res, 66306);
        }
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        if let Ok(res) = result {
            assert_eq!(res, 45000);
        }
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        if let Ok(res) = result {
            assert_eq!(res, 195292);
        }
    }
}
