#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    Solution::dfs(&mut grid, i as i32, j as i32);
                    islands += 1;
                }
            }
        }

        islands
    }

    pub fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] != '1'
        {
            return;
        }

        grid[i as usize][j as usize] = '2';

        Solution::dfs(grid, i + 1, j);
        Solution::dfs(grid, i - 1, j);
        Solution::dfs(grid, i, j + 1);
        Solution::dfs(grid, i, j - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_pass_basic_case() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }
}
