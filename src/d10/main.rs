use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
};

use crate::aoc_test;

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    // let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    // let lines = line_iter.collect::<Vec<String>>();

    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    fn dfs(
        y: i32,
        x: i32,
        grid: &Vec<Vec<u8>>,
        seen: &mut HashMap<(i32, i32), bool>,
        elevation: u8,
    ) {
        if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[0].len() as i32 {
            return;
        }
        let cell = grid[y as usize][x as usize];

        dbg!(cell, elevation);
        if seen.contains_key(&(y, x)) || cell <= elevation {
            return;
        }
        // only proceed if in bounds, higher and yet unseen

        seen.insert((y, x), cell == b'9');
        if cell == b'9' {
            return;
        }

        dfs(y - 1, x, grid, seen, elevation + 1);
        dfs(y + 1, x, grid, seen, elevation + 1);
        dfs(y, x - 1, grid, seen, elevation + 1);
        dfs(y, x + 1, grid, seen, elevation + 1);
    }

    let mut nines = HashSet::<(i32, i32)>::new();
    for y in 0..h {
        for x in 0..w {
            let cell = grid[y as usize][x as usize];
            if cell != b'0' {
                continue;
            }

            let mut seen: HashMap<(i32, i32), bool> = HashMap::new();
            dfs(y, x, &grid, &mut seen, b'0' - 1);

            seen.retain(|_, v| *v);
            nines.extend(seen.keys().cloned());
        }
    }

    Ok(nines.len())
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    // let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    // let lines = line_iter.collect::<Vec<String>>();

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_solution1() {
        let input = utils::file_line_iter("./text/10");
        assert_eq!(solution1(input).unwrap(), 0);
    }

    #[test]
    fn test_solution2() {
        let input = utils::file_line_iter("./text/10");
        assert_eq!(solution2(input).unwrap(), 0);
    }
}
