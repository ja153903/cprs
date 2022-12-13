#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};

fn fetch_data(is_sample: bool) -> &'static str {
    if is_sample {
        include_str!("./data/day12_sample.txt")
    } else {
        include_str!("./data/day12.txt")
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(is_sample: bool) -> i32 {
    let grid = fetch_data(is_sample)
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut start: (i32, i32) = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
                break;
            }
        }
    }

    queue.push_back((start.0, start.1, 0));
    visited.insert(start);

    while !queue.is_empty() {
        if let Some(front) = queue.pop_front() {
            let (x, y, steps) = front;
            let current = grid[x as usize][y as usize];
            let current_charcode = match current {
                'S' => 'a' as u32,
                'E' => 'z' as u32,
                _ => current as u32,
            };

            if grid[x as usize][y as usize] == 'E' {
                return steps;
            }

            for (dx, dy) in DIRECTIONS {
                let nx = x + dx;
                let ny = y + dy;

                // out of bounds
                if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                    continue;
                }

                if visited.contains(&(nx, ny)) {
                    continue;
                }

                let next = grid[nx as usize][ny as usize];
                let next_charcode = match next {
                    'S' => 'a' as u32,
                    'E' => 'z' as u32,
                    _ => next as u32,
                };

                let (diff, is_overflow) = next_charcode.overflowing_sub(current_charcode);
                let at_most_one_away = diff <= 1;

                if is_overflow || at_most_one_away {
                    queue.push_back((nx, ny, steps + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    0
}

fn part2(is_sample: bool) -> i32 {
    let grid = fetch_data(is_sample)
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' || grid[i][j] == 'a' {
                queue.push_back((i as i32, j as i32, 0));
                visited.insert((i as i32, j as i32));
            }
        }
    }

    while !queue.is_empty() {
        if let Some(front) = queue.pop_front() {
            let (x, y, steps) = front;
            let current = grid[x as usize][y as usize];
            let current_charcode = match current {
                'S' => 'a' as u32,
                'E' => 'z' as u32,
                _ => current as u32,
            };

            if grid[x as usize][y as usize] == 'E' {
                return steps;
            }

            for (dx, dy) in DIRECTIONS {
                let nx = x + dx;
                let ny = y + dy;

                // out of bounds
                if nx < 0 || ny < 0 || nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                    continue;
                }

                if visited.contains(&(nx, ny)) {
                    continue;
                }

                let next = grid[nx as usize][ny as usize];
                let next_charcode = match next {
                    'S' => 'a' as u32,
                    'E' => 'z' as u32,
                    _ => next as u32,
                };

                let (diff, is_overflow) = next_charcode.overflowing_sub(current_charcode);
                let at_most_one_away = diff <= 1;

                if is_overflow || at_most_one_away {
                    queue.push_back((nx, ny, steps + 1));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn run_part1_sample() {
        let result = part1(true);
        assert_eq!(result, 31);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(false);
        assert_eq!(result, 383);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(true);
        assert_eq!(result, 29);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(false);
        assert_eq!(result, 377);
    }
}
