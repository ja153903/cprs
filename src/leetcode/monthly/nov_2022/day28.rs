#![allow(dead_code)]

use std::collections::BTreeMap;

struct Solution;

struct WinLossRecord {
    pub win: i32,
    pub loss: i32,
}

impl WinLossRecord {
    pub fn new(win: i32, loss: i32) -> Self {
        Self { win, loss }
    }
}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![], vec![]];
        let mut by_player: BTreeMap<i32, WinLossRecord> = BTreeMap::new();

        for mtch in matches.iter() {
            let winner = mtch[0];
            let loser = mtch[1];

            by_player
                .entry(winner)
                .and_modify(|win_loss_record| win_loss_record.win += 1)
                .or_insert(WinLossRecord::new(1, 0));

            by_player
                .entry(loser)
                .and_modify(|win_loss_record| win_loss_record.loss += 1)
                .or_insert(WinLossRecord::new(0, 1));
        }

        for (&key, value) in by_player.iter() {
            if value.loss == 0 {
                result[0].push(key);
            }

            if value.loss == 1 {
                result[1].push(key);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn should_pass_sample_case() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];

        let expected = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];

        assert_eq!(Solution::find_winners(matches), expected);
    }
}
