use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::aoc_test;

aoc_test!(day: d12, version: main, part1: 1377008, part2: 815788);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    // let lines = line_iter.collect::<Vec<String>>();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut total_price = 0;

    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(y, x)) {
                continue;
            }

            let plant = grid[y as usize][x as usize];
            let mut region = HashSet::<(i32, i32)>::new();

            let perimeter = search_regions_and_perimeters(y, x, plant, &grid, &mut region);
            total_price += perimeter as usize * region.len();

            visited.extend(region);
        }
    }

    Ok(total_price)
}

fn search_regions_and_perimeters(
    y: i32,
    x: i32,
    plant: u8,
    grid: &Vec<Vec<u8>>,
    region: &mut HashSet<(i32, i32)>,
) -> u64 {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    if region.contains(&(y, x)) {
        return 0;
    }
    if y < 0 || y >= h || x < 0 || x >= w || grid[y as usize][x as usize] != plant {
        return 1;
    }

    region.insert((y, x));

    search_regions_and_perimeters(y - 1, x, plant, grid, region)
        + search_regions_and_perimeters(y + 1, x, plant, grid, region)
        + search_regions_and_perimeters(y, x - 1, plant, grid, region)
        + search_regions_and_perimeters(y, x + 1, plant, grid, region)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    // let lines = line_iter.collect::<Vec<String>>();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut total_price = 0;

    for y in 0..h {
        for x in 0..w {
            if visited.contains(&(y, x)) {
                continue;
            }

            let plant = grid[y as usize][x as usize];
            let mut region = HashSet::<(i32, i32)>::new();
            let mut fences = HashMap::<Fence, Vec<i32>>::new();

            search_regions_and_sides(y, x, plant, &grid, &mut region, &mut fences, (0, 0));

            // Number of segments = number of gaps + 1
            let sides: usize = fences
                .values_mut()
                .map(|f| {
                    f.sort();
                    1 + f
                        .iter()
                        .tuple_windows()
                        .filter(|&(f1, f2)| *f1 + 1 != *f2)
                        .count()
                })
                .sum();

            total_price += sides * region.len();
            visited.extend(region);
        }
    }

    Ok(total_price)
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fence {
    Vertical(
        i32, /* coordinate of fence in half units */
        i32, /* direction from which the fence was found */
    ),
    Horizontal(i32, i32),
}

fn search_regions_and_sides(
    y: i32,
    x: i32,
    plant: u8,
    grid: &Vec<Vec<u8>>,
    region: &mut HashSet<(i32, i32)>,
    fences: &mut HashMap<Fence, Vec<i32>>,
    last_dir: (i32, i32),
) {
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    // if visited this plant, return
    if region.contains(&(y, x)) {
        return;
    }

    // if not visited but out of bounds or wrong plant, found fence
    if y < 0 || y >= h || x < 0 || x >= w || grid[y as usize][x as usize] != plant {
        // If fence direction is vertical
        if matches!(last_dir, (0, _)) {
            // for given vertical x slice, count all the y
            fences
                .entry(Fence::Vertical(x * 2 - last_dir.1, last_dir.1))
                .or_default()
                .push(y - last_dir.0);
        } else {
            // for given horizontal y slice, count all the x
            fences
                .entry(Fence::Horizontal(y * 2 - last_dir.0, last_dir.0))
                .or_default()
                .push(x - last_dir.1);
        }
        return;
    }

    region.insert((y, x));

    search_regions_and_sides(y - 1, x, plant, grid, region, fences, (-1, 0));
    search_regions_and_sides(y + 1, x, plant, grid, region, fences, (1, 0));
    search_regions_and_sides(y, x - 1, plant, grid, region, fences, (0, -1));
    search_regions_and_sides(y, x + 1, plant, grid, region, fences, (0, 1));
}
