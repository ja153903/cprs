#![allow(dead_code)]

#[derive(Debug, Clone)]
struct NestedInteger {
    int_value: Option<i32>,
    list_value: Option<Vec<NestedInteger>>,
}

fn fetch_data(is_sample: bool) -> &'static str {
    if is_sample {
        include_str!("./data/day13_sample.txt")
    } else {
        include_str!("./data/day13.txt")
    }
}

// TODO: Solve this
fn parse_data(input: &str) -> Vec<NestedInteger> {
    vec![]
}

/// Comparison logic:
///
/// if both values are integers, then the following comparison has to be made:
/// - if left < right, then we return true
/// - if left > right, then we return false
/// - if left == right, then we move on to the next value
///
/// if one value is an integer and the other is a vector, we cast the integer as a vector and apply the logic
/// for vectors
///
/// if both values are vectors, then the following logic applies as we go through each element
/// - the same comparison logic for numbers applies here as well
/// - if the left list runs out of elements before finding a pair in the right order, then we return true
/// - if the right list runs out of elements before finding a pair in the right order, then we return false
fn is_right_order(pair: (Vec<NestedInteger>, Vec<NestedInteger>)) -> bool {
    let left = pair.0;
    let right = pair.1;

    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        let left_val = left[left_idx].clone();
        let right_val = right[right_idx].clone();

        let left_is_int = left_val.int_value.is_some();
        let right_is_int = right_val.int_value.is_some();

        match (left_is_int, right_is_int) {
            (true, true) => {
                if let Some(left_int) = left_val.int_value {
                    if let Some(right_int) = right_val.int_value {
                        match left_int.cmp(&right_int) {
                            std::cmp::Ordering::Less => {
                                return false;
                            }
                            std::cmp::Ordering::Greater => {
                                return true;
                            }
                            std::cmp::Ordering::Equal => {}
                        }
                    }
                }
            }
            (false, false) => {
                if let Some(left_list) = left_val.list_value {
                    if let Some(right_list) = right_val.list_value {
                        return is_right_order((left_list, right_list));
                    }
                }
            }
            (false, true) | (true, false) => {}
        }

        left_idx += 1;
        right_idx += 1;
    }

    left_idx == left.len()
}

fn part1(is_sample: bool) -> i32 {
    fetch_data(is_sample)
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair.split('\n').map(parse_data);
            match (iter.next(), iter.next()) {
                (Some(a), Some(b)) => (a, b),
                _ => panic!("This should not have happened"),
            }
        })
        .enumerate()
        .map(|(index, pair)| {
            if is_right_order(pair) {
                index as i32 + 1
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
