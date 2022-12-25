#![allow(dead_code)]

use std::io;

use crate::advent_of_code::helpers::file::read_lines;

enum Game {
    SCISSORS,
    PAPER,
    ROCK,
}

enum Decision {
    LOSE,
    WIN,
    DRAW,
}

fn to_game_enum(ch: &str) -> Game {
    match ch {
        "X" | "A" => Game::ROCK,
        "Y" | "B" => Game::PAPER,
        _ => Game::SCISSORS,
    }
}

fn to_decision_enum(ch: &str) -> Decision {
    match ch {
        "X" => Decision::LOSE,
        "Y" => Decision::DRAW,
        _ => Decision::WIN,
    }
}

fn from_game_enum_to_score(game: Game) -> i32 {
    match game {
        Game::ROCK => 1,
        Game::PAPER => 2,
        Game::SCISSORS => 3,
    }
}

fn part1(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result: i32 = 0;
            for line in lines {
                if let Ok(round) = line {
                    let split = round.split_whitespace();
                    let parts = split.take(2).collect::<Vec<&str>>();

                    let enemy_choice = to_game_enum(parts[0]);
                    let my_choice = to_game_enum(parts[1]);

                    result += match my_choice {
                        Game::ROCK => 1,
                        Game::PAPER => 2,
                        Game::SCISSORS => 3,
                    };

                    result += match (my_choice, enemy_choice) {
                        (Game::ROCK, Game::SCISSORS)
                        | (Game::PAPER, Game::ROCK)
                        | (Game::SCISSORS, Game::PAPER) => 6,
                        (Game::ROCK, Game::ROCK)
                        | (Game::PAPER, Game::PAPER)
                        | (Game::SCISSORS, Game::SCISSORS) => 3,
                        _ => 0,
                    };
                }
            }

            Ok(result)
        }
    }
}

fn part2(path: &str) -> io::Result<i32> {
    match read_lines(path) {
        Err(e) => Err(e),
        Ok(lines) => {
            let mut result: i32 = 0;

            for line in lines {
                if let Ok(round) = line {
                    let split = round.split_whitespace();
                    let parts = split.take(2).collect::<Vec<&str>>();

                    let enemy_choice = to_game_enum(parts[0]);
                    let decision = to_decision_enum(parts[1]);

                    result += match decision {
                        Decision::DRAW => 3 + from_game_enum_to_score(enemy_choice),
                        Decision::LOSE => match enemy_choice {
                            Game::PAPER => from_game_enum_to_score(Game::ROCK),
                            Game::ROCK => from_game_enum_to_score(Game::SCISSORS),
                            _ => from_game_enum_to_score(Game::PAPER),
                        },
                        Decision::WIN => {
                            6 + match enemy_choice {
                                Game::PAPER => from_game_enum_to_score(Game::SCISSORS),
                                Game::ROCK => from_game_enum_to_score(Game::PAPER),
                                _ => from_game_enum_to_score(Game::ROCK),
                            }
                        }
                    };
                }
            }

            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    const SAMPLE_DATA: &str = "./src/advent_of_code/aoc2022/data/day2_sample.txt";
    const DATA: &str = "./src/advent_of_code/aoc2022/data/day2.txt";

    #[test]
    pub fn run_part1_sample() {
        let result = part1(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 15);
    }

    #[test]
    pub fn run_part1() {
        let result = part1(DATA);
        assert_eq!(result.unwrap(), 8933);
    }

    #[test]
    pub fn run_part2_sample() {
        let result = part2(SAMPLE_DATA);
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    pub fn run_part2() {
        let result = part2(DATA);
        assert_eq!(result.unwrap(), 11998);
    }
}
