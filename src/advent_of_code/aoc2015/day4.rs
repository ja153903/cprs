#![allow(dead_code)]

use md5::compute;

fn part1() -> i32 {
    let key = String::from("iwrupvqb");
    let mut i = 1;
    loop {
        let hash = format!("{:x}", compute(format!("{}{}", key, i)));
        if hash.starts_with("00000") {
            return i;
        }

        i += 1;
    }
}

fn part2() -> i32 {
    let key = String::from("iwrupvqb");
    let mut i = 1;
    loop {
        let hash = format!("{:x}", compute(format!("{}{}", key, i)));
        if hash.starts_with("000000") {
            return i;
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[ignore = "takes too long"]
    #[test]
    pub fn run_part1() {
        let result = part1();
        assert_eq!(result, 346386);
    }

    #[ignore = "takes too long"]
    #[test]
    pub fn run_part2() {
        let result = part2();
        assert_eq!(result, 9958218);
    }
}
