use itertools::{chain, Itertools};
use regex::Regex;
use std::error::Error;

use crate::aoc_test;

aoc_test!(day: d04, version: main, part1: 2483, part2: 1925);

pub fn count_matches(text: &str, pattern: &str) -> u32 {
    let re = Regex::new(pattern).unwrap();
    re.find_iter(text).count() as u32
}

pub fn solution1(mut line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let dirs: [(i32, i32); 8] = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let s = "XMAS".as_bytes();

    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            for dir in dirs {
                let (mut i, mut j) = (0, 0);
                let mut si = 0;

                while y + i >= 0
                    && y + i < h
                    && x + j >= 0
                    && x + j < w
                    && si < s.len()
                    && grid[(y + i) as usize][(x + j) as usize] == s[si]
                {
                    i += dir.0;
                    j += dir.1;
                    si += 1;
                }

                if si == s.len() {
                    count += 1;
                }
            }
        }
    }

    // make test a grid xy
    // for each cell, if ch is X
    // for each 8 direction, keep direction and search MAS

    Ok(count)
}

pub fn solution2(mut line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let dirs: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut count = 0;

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if grid[y as usize][x as usize] != b'A' {
                continue;
            }

            let mut checks = 0;

            for dir in dirs {
                let cell = grid[(y + dir.0) as usize][(x + dir.1) as usize];

                if cell != b'S' && cell != b'M' {
                    continue;
                }

                let opposite_cell = grid[(y - dir.0) as usize][(x - dir.1) as usize];
                if cell == b'M' && opposite_cell == b'S' {
                    checks += 1;
                }
            }

            if checks == 2 {
                count += 1;
            }
        }
    }

    // make test a grid xy
    // for each cell, if ch is X
    // for each 8 direction, keep direction and search MAS

    Ok(count)
}

// pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {}
