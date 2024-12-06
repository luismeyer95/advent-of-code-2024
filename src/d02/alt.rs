use crate::aoc_test;
use itertools::{chain, Itertools};
use std::error::Error;

aoc_test!(day: d02, version: main, part1: 359, part2: 418);

pub fn is_valid_level(nums: &[isize]) -> bool {
    let sign = nums[1] - nums[0];
    if sign == 0 {
        return false;
    }

    nums.iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .all(|diff| diff * sign > 0 && diff.abs() >= 1 && diff.abs() <= 3)
}

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    Ok(line_iter
        .map(|ln| {
            ln.split_whitespace()
                .flat_map(|s| s.parse::<isize>().ok())
                .collect_vec()
        })
        .filter(|nums| is_valid_level(nums))
        .count())
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    Ok(line_iter
        .map(|ln| {
            ln.split_whitespace()
                .flat_map(|s| s.parse::<isize>().ok())
                .collect_vec()
        })
        .filter(|nums| {
            is_valid_level(nums)
                || (0..nums.len())
                    .map(|i| chain(&nums[..i], &nums[i + 1..]).copied())
                    .any(|slice| is_valid_level(&slice.collect_vec()))
        })
        .count())
}
