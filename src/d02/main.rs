use itertools::{chain, Itertools};
use std::error::Error;

use crate::aoc_test;

aoc_test!(day: d02, version: main, part1: 359, part2: 418);

pub fn is_valid_level(nums: &[isize]) -> bool {
    let sign = nums[1] - nums[0];
    if sign == 0 {
        return false;
    }

    for (i, (a, b)) in nums.iter().tuple_windows().enumerate() {
        let diff = b - a;
        if diff * sign < 0 || diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}

pub fn is_valid_level_2(nums: &[isize]) -> bool {
    let mut result = is_valid_level(nums);

    for i in 0..nums.len() {
        let new_nums = chain(&nums[..i], &nums[i + 1..])
            .copied()
            .collect::<Vec<isize>>();

        result = result || is_valid_level(&new_nums);
    }

    result
}

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let mut count = 0;
    for line in line_iter {
        let nums = line.split_whitespace().map(|s| s.parse::<isize>().unwrap());
        if is_valid_level(&nums.collect::<Vec<_>>()) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let mut count = 0;
    for line in line_iter {
        let nums = line.split_whitespace().map(|s| s.parse::<isize>().unwrap());

        if is_valid_level_2(&nums.collect::<Vec<_>>()) {
            count += 1;
        }
    }

    Ok(count)
}
