use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::aoc_test;

aoc_test!(day: d8, version: main, part1: 426, part2: 1359);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    // iter grid and store map<freq, vec<coords>>
    // for each frequency
    // for each permutation of pairs of coords
    // project 1st coord past the 2nd
    // if in bounds, store in set
    // return set count
    //
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut map: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..h {
        for x in 0..w {
            let cell = grid[y as usize][x as usize];
            if !cell.is_ascii_alphanumeric() {
                continue;
            }

            map.entry(cell).or_default().push((y, x));
        }
    }

    for coords in map.values() {
        for pair in coords.iter().permutations(2) {
            let (y1, x1) = pair[0];
            let (y2, x2) = pair[1];
            let dy = y2 - y1;
            let dx = x2 - x1;
            let y3 = y1 + 2 * dy;
            let x3 = x1 + 2 * dx;
            if y3 >= 0 && y3 < h && x3 >= 0 && x3 < w {
                antinode_locations.insert((y3, x3));
            }
        }
    }

    Ok(antinode_locations.len())
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut map: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut antinode_locations: HashSet<(i32, i32)> = HashSet::new();

    for y in 0..h {
        for x in 0..w {
            let cell = grid[y as usize][x as usize];
            if !cell.is_ascii_alphanumeric() {
                continue;
            }

            map.entry(cell).or_default().push((y, x));
        }
    }

    for coords in map.values() {
        for pair in coords.iter().permutations(2) {
            let (y1, x1) = *pair[0];
            let (y2, x2) = *pair[1];
            let dy = y2 - y1;
            let dx = x2 - x1;

            let mut y = y1;
            let mut x = x1;
            while y >= 0 && y < h && x >= 0 && x < w {
                antinode_locations.insert((y, x));
                y += dy;
                x += dx;
            }
        }
    }

    Ok(antinode_locations.len())
}
