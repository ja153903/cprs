#![allow(dead_code)]

use crate::advent_of_code::helpers::file::read_lines;
use std::collections::{HashMap, VecDeque};
use std::io;

// TODO: Figure out how to use include_str! macro for reading the file input
// NOTE: This might clean up my implementation a little bit

#[derive(Debug)]
struct MonkeyOps {
    a: String,
    b: String,
    op: String,
    divmod: i64,
    if_true: i64,
    if_false: i64,
}

impl MonkeyOps {
    fn new(a: &str, b: &str, op: &str, divmod: i64, if_true: i64, if_false: i64) -> Self {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            op: op.to_string(),
            divmod,
            if_true,
            if_false,
        }
    }

    fn operation(&self, item: i64) -> i64 {
        let a = if self.a.as_str() == "old" {
            item
        } else {
            self.a.parse::<i64>().unwrap()
        };

        let b = if self.b.as_str() == "old" {
            item
        } else {
            self.b.parse::<i64>().unwrap()
        };

        match self.op.as_ref() {
            "+" => a + b,
            _ => a * b,
        }
    }

    fn test(&self, new: i64) -> (i64, i64) {
        if new % self.divmod == 0 {
            (new, self.if_true)
        } else {
            (new, self.if_false)
        }
    }
}

fn part1(path: &str) -> io::Result<i64> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let lines = lines.flatten().collect::<Vec<String>>();
            let mut monkey_ops_by_num: HashMap<i64, MonkeyOps> = HashMap::new();
            let mut items_by_monkey: Vec<VecDeque<i64>> = Vec::new();

            let mut idx = 0;

            while idx < lines.len() {
                if lines[idx].is_empty() {
                    idx += 1;
                } else {
                    let current_monkey = lines[idx]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .split(':')
                        .next()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Something went wrong with parsing");

                    let current_items = lines[idx + 1]
                        .split(": ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .map(|val| val.parse::<i64>().unwrap())
                        .collect::<VecDeque<i64>>();

                    let mut equation = lines[idx + 2]
                        .trim()
                        .split(": ")
                        .last()
                        .unwrap()
                        .split_whitespace();

                    equation.next();
                    equation.next();

                    let a = equation.next().unwrap();
                    let op = equation.next().unwrap();
                    let b = equation.next().unwrap();

                    let divmod = lines[idx + 3]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse number");

                    let if_true = lines[idx + 4]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse monkey");

                    let if_false = lines[idx + 5]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse monkey");

                    let monkey_ops = MonkeyOps::new(a, b, op, divmod, if_true, if_false);

                    monkey_ops_by_num
                        .entry(current_monkey)
                        .or_insert(monkey_ops);

                    items_by_monkey.push(current_items);

                    idx += 7;
                }
            }

            // now that we've initialized the array, we can start the processing
            let n = items_by_monkey.len();
            let mut freq: Vec<i64> = vec![0; items_by_monkey.len()];

            for _ in 0..20 {
                for monkey in 0..n {
                    if let Some(monkey_op) = monkey_ops_by_num.get(&(monkey as i64)) {
                        let mut items_to_push: Vec<(i64, i64)> = vec![];

                        if let Some(items) = items_by_monkey.get_mut(monkey) {
                            let size = items.len();
                            freq[monkey as usize] += size as i64;
                            for _ in 0..size {
                                let item = items.pop_front().unwrap();
                                let result = monkey_op.operation(item);
                                let (new_item, target) = monkey_op.test(result / 3);

                                items_to_push.push((new_item, target));
                            }
                        }

                        for (new_item, target) in items_to_push.iter() {
                            items_by_monkey[*target as usize].push_back(*new_item);
                        }
                    }
                }
            }

            freq.sort_by(|a, b| b.cmp(a));

            Ok(freq[0] * freq[1])
        }
    }
}

fn part2(path: &str) -> io::Result<i64> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let lines = lines.flatten().collect::<Vec<String>>();
            let mut monkey_ops_by_num: HashMap<i64, MonkeyOps> = HashMap::new();
            let mut items_by_monkey: Vec<VecDeque<i64>> = Vec::new();

            let mut idx = 0;

            while idx < lines.len() {
                if lines[idx].is_empty() {
                    idx += 1;
                } else {
                    let current_monkey = lines[idx]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .split(':')
                        .next()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Something went wrong with parsing");

                    let current_items = lines[idx + 1]
                        .split(": ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .map(|val| val.parse::<i64>().unwrap())
                        .collect::<VecDeque<i64>>();

                    let mut equation = lines[idx + 2]
                        .trim()
                        .split(": ")
                        .last()
                        .unwrap()
                        .split_whitespace();

                    equation.next();
                    equation.next();

                    let a = equation.next().unwrap();
                    let op = equation.next().unwrap();
                    let b = equation.next().unwrap();

                    let divmod = lines[idx + 3]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse number");

                    let if_true = lines[idx + 4]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse monkey");

                    let if_false = lines[idx + 5]
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<i64>()
                        .expect("Could not parse monkey");

                    let monkey_ops = MonkeyOps::new(a, b, op, divmod, if_true, if_false);

                    monkey_ops_by_num
                        .entry(current_monkey)
                        .or_insert(monkey_ops);

                    items_by_monkey.push(current_items);

                    idx += 7;
                }
            }

            // now that we've initialized the array, we can start the processing
            let n = items_by_monkey.len();
            let mut freq: Vec<i64> = vec![0; items_by_monkey.len()];
            let mut divmod = 1;
            for monkey in 0..n {
                if let Some(monkey_op) = monkey_ops_by_num.get(&(monkey as i64)) {
                    divmod *= monkey_op.divmod;
                }
            }

            for _ in 0..10000 {
                for monkey in 0..n {
                    let mut items_to_push: Vec<(i64, i64)> = vec![];
                    if let Some(monkey_op) = monkey_ops_by_num.get(&(monkey as i64)) {
                        if let Some(items) = items_by_monkey.get_mut(monkey) {
                            let size = items.len();
                            freq[monkey as usize] += size as i64;
                            for _ in 0..size {
                                let item = items.pop_front().unwrap();
                                let result = monkey_op.operation(item) % divmod;
                                let (new_item, target) = monkey_op.test(result);

                                items_to_push.push((new_item, target));
                            }
                        }
                    }

                    for (new_item, target) in items_to_push.iter() {
                        items_by_monkey[*target as usize].push_back(*new_item);
                    }
                }
            }

            freq.sort_by(|a, b| b.cmp(a));

            Ok(freq.iter().take(2).product())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day11_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day11.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 10605);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 54054);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 2713310158);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 14314925001);
    }
}
