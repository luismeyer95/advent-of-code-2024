use itertools::Itertools;
use std::{collections::HashSet, error::Error};

use crate::aoc_test;

aoc_test!(day: d10, version: main, part1: 688, part2: 1459);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    fn dfs(y: i32, x: i32, grid: &Vec<Vec<u8>>, seen: &mut HashSet<(i32, i32)>, last_height: u8) {
        // if cell not in bounds, stop
        if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[0].len() as i32 {
            return;
        }

        // if cell not higher than the last, stop
        let cell = grid[y as usize][x as usize];
        if cell != last_height + 1 {
            return;
        }

        // if found a 9, save it and stop
        if cell == b'9' {
            seen.insert((y, x));
            return;
        }

        dfs(y - 1, x, grid, seen, last_height + 1);
        dfs(y + 1, x, grid, seen, last_height + 1);
        dfs(y, x - 1, grid, seen, last_height + 1);
        dfs(y, x + 1, grid, seen, last_height + 1);
    }

    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = grid[y as usize][x as usize];
            if cell != b'0' {
                continue;
            }

            let mut seen = HashSet::new();
            dfs(y, x, &grid, &mut seen, b'0' - 1);

            count += seen.len();
        }
    }

    Ok(count)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    fn dfs(y: i32, x: i32, grid: &Vec<Vec<u8>>, last_height: u8) -> usize {
        // if cell not in bounds, stop
        if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[0].len() as i32 {
            return 0;
        }

        // if cell not higher than the last, stop
        let cell = grid[y as usize][x as usize];
        if cell != last_height + 1 {
            return 0;
        }

        // if found a 9, save it and stop
        if cell == b'9' {
            return 1;
        }

        let mut count = 0;
        count += dfs(y - 1, x, grid, last_height + 1);
        count += dfs(y + 1, x, grid, last_height + 1);
        count += dfs(y, x - 1, grid, last_height + 1);
        count += dfs(y, x + 1, grid, last_height + 1);
        count
    }

    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let cell = grid[y as usize][x as usize];
            if cell != b'0' {
                continue;
            }

            count += dfs(y, x, &grid, b'0' - 1);
        }
    }

    Ok(count)
}
