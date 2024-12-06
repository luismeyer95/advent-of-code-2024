use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::aoc_test;

aoc_test!(day: d06, version: main, part1: 5444, part2: 1946);

pub fn turn_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        _ => panic!(),
    }
}

pub fn resolve_dir(ch: u8) -> Option<(i32, i32)> {
    match ch {
        b'v' => (1, 0).into(),
        b'^' => (-1, 0).into(),
        b'>' => (0, 1).into(),
        b'<' => (0, -1).into(),
        _ => None,
    }
}

pub fn solution1(mut line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut count = 0;

    for y in 0..h {
        for x in 0..w {
            if let Some(mut dir) = resolve_dir(grid[y as usize][x as usize]) {
                let mut seen: HashSet<(i32, i32)> = HashSet::new();
                seen.insert((y, x));

                let (mut i, mut j) = (y, x);

                while i + dir.0 < h && i + dir.0 >= 0 && j + dir.1 < w && j + dir.1 >= 0 {
                    let next = grid[(i + dir.0) as usize][(j + dir.1) as usize];
                    if next == b'#' {
                        dir = turn_right(dir);
                        continue;
                    }
                    i += dir.0;
                    j += dir.1;
                    seen.insert((i, j));
                }

                count = seen.len();
                break;
            }
        }
    }

    Ok(count)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let mut grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut count = 0;

    let mut start = (0, 0);
    let mut start_dir = (0, 0);
    for y in 0..h {
        for x in 0..w {
            if let Some(guard_dir) = resolve_dir(grid[y as usize][x as usize]) {
                start = (y, x);
                start_dir = guard_dir;
                break;
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            // If empty space,
            if grid[y as usize][x as usize] != b'.' {
                continue;
            }

            // Create obstacle
            grid[y as usize][x as usize] = b'#';

            let mut seen: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
            let (mut i, mut j) = start;
            let mut dir = start_dir;
            seen.insert((i, j), dir);

            // Run guard sim, increment count if cell seen twice in same direction
            while i + dir.0 < h && i + dir.0 >= 0 && j + dir.1 < w && j + dir.1 >= 0 {
                let next = grid[(i + dir.0) as usize][(j + dir.1) as usize];
                if next == b'#' {
                    dir = turn_right(dir);
                    continue;
                }
                i += dir.0;
                j += dir.1;
                if seen.contains_key(&(i, j)) && seen[&(i, j)] == dir {
                    count += 1;
                    break;
                }
                seen.insert((i, j), dir);
            }

            grid[y as usize][x as usize] = b'.';
        }
    }

    Ok(count)
}

// pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
//     let mut grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
//     let h = grid.len() as i32;
//     let w = grid[0].len() as i32;
//
//     let mut start = (0, 0);
//     let mut dir = (0, 0);
//     for y in 0..h {
//         for x in 0..w {
//             if let Some(guard_dir) = resolve_dir(grid[y as usize][x as usize]) {
//                 start = (y, x);
//                 dir = guard_dir;
//                 break;
//             }
//         }
//     }
//
//     let (mut y, mut x) = start;
//     let mut looping_pos = HashSet::new();
//
//     // Run along the initial guard path
//     while y + dir.0 < h && y + dir.0 >= 0 && x + dir.1 < w && x + dir.1 >= 0 {
//         let (y_next, x_next) = ((y + dir.0) as usize, (x + dir.1) as usize);
//
//         if grid[y_next][x_next] == b'#' {
//             dir = turn_right(dir);
//             continue;
//         }
//
//         // No obstacle in front, but what if there is?
//         // Run guard sim for this case, increment if it loops
//         grid[y_next][x_next] = b'#';
//         if guard_sim_is_looping(&grid, (y, x), dir) {
//             looping_pos.insert((y_next, x_next));
//         }
//         // Reset
//         grid[y_next][x_next] = b'.';
//
//         y += dir.0;
//         x += dir.1;
//     }
//
//     Ok(looping_pos.len())
// }
//
// fn guard_sim_is_looping(grid: &[Vec<u8>], (mut y, mut x): (i32, i32), mut dir: (i32, i32)) -> bool {
//     let h = grid.len() as i32;
//     let w = grid[0].len() as i32;
//
//     let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
//     seen.insert((y, x, dir.0, dir.1));
//
//     // Run guard sim, increment count if cell seen twice in same direction
//     while y + dir.0 < h && y + dir.0 >= 0 && x + dir.1 < w && x + dir.1 >= 0 {
//         let next = grid[(y + dir.0) as usize][(x + dir.1) as usize];
//         if next == b'#' {
//             dir = turn_right(dir);
//             continue;
//         }
//         y += dir.0;
//         x += dir.1;
//         if seen.contains(&(y, x, dir.0, dir.1)) {
//             return true;
//         }
//         seen.insert((y, x, dir.0, dir.1));
//     }
//
//     false
// }
