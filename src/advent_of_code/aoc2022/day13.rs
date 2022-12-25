#![allow(dead_code)]

// TODO: it would make sense to install that eval crate here

#[derive(Debug, Clone)]
enum DynNestedIntegerType {
    List(Vec<DynNestedIntegerType>),
    Integer(i32),
}

fn fetch_data(is_sample: bool) -> &'static str {
    if is_sample {
        include_str!("./data/day13_sample.txt")
    } else {
        include_str!("./data/day13.txt")
    }
}

// TODO: Figure out a way to parse this line
fn parse_line(line: &str) -> Vec<DynNestedIntegerType> {
    let mut items = vec![];
    let mut stack: Vec<i32> = vec![];
    let mut current_number = 0;

    for ch in line.chars() {
        if ch.is_ascii_digit() {
            current_number = current_number * 10 + ch.to_digit(10).unwrap();
        }
    }

    items
}

fn parse_block(block: &str) -> Vec<Vec<DynNestedIntegerType>> {
    block
        .split('\n')
        .map(parse_line)
        .collect::<Vec<Vec<DynNestedIntegerType>>>()
}

fn get_index(pair: Vec<Vec<DynNestedIntegerType>>) -> (bool, bool) {
    let left = pair[0].clone();
    let right = pair[1].clone();

    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        let left_node = left[left_idx].clone();
        let right_node = right[right_idx].clone();

        match (left_node, right_node) {
            (
                DynNestedIntegerType::Integer(left_value),
                DynNestedIntegerType::Integer(right_value),
            ) => {
                if left_value == right_value {
                    left_idx += 1;
                    right_idx += 1;

                    continue;
                }

                return (left_value < right_value, false);
            }
            (DynNestedIntegerType::List(left_value), DynNestedIntegerType::List(right_value)) => {
                let (is_right_order, is_equal) = get_index(vec![left_value, right_value]);

                if is_equal {
                    left_idx += 1;
                    right_idx += 1;

                    continue;
                }

                return (is_right_order, false);
            }
            (
                DynNestedIntegerType::Integer(left_value),
                DynNestedIntegerType::List(right_value),
            ) => {
                let (is_right_order, is_equal) = get_index(vec![
                    vec![DynNestedIntegerType::Integer(left_value)],
                    right_value,
                ]);

                if is_equal {
                    left_idx += 1;
                    right_idx += 1;

                    continue;
                }

                return (is_right_order, false);
            }
            (
                DynNestedIntegerType::List(left_value),
                DynNestedIntegerType::Integer(right_value),
            ) => {
                let (is_right_order, is_equal) = get_index(vec![
                    left_value,
                    vec![DynNestedIntegerType::Integer(right_value)],
                ]);

                if is_equal {
                    left_idx += 1;
                    right_idx += 1;

                    continue;
                }

                return (is_right_order, false);
            }
        };
    }

    let is_equal = left_idx == left.len() && right_idx == right.len();

    (left_idx == left.len(), is_equal)
}

fn part1(is_sample: bool) -> i32 {
    fetch_data(is_sample)
        .split("\n\n")
        .map(parse_block)
        .enumerate()
        .map(|(idx, pair)| {
            let (is_right_order, is_equal) = get_index(pair);
            if is_right_order && !is_equal {
                idx as i32 + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(is_sample: bool) -> i32 {
    let data = fetch_data(is_sample);
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1_sample() {
        let result = part1(true);
        assert_eq!(result, 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part1() {
        let result = part1(false);
        assert_eq!(result, 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part2_sample() {
        let result = part2(true);
        assert_eq!(result, 0);
    }

    #[ignore = "no implementation"]
    #[test]
    pub fn run_part2() {
        let result = part2(false);
        assert_eq!(result, 0);
    }
}
