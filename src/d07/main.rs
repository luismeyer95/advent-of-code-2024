use itertools::Itertools;

use crate::aoc_test;
use std::error::Error;

aoc_test!(day: d07, version: main, part1: 3245122495150, part2: 105517128211543);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<u64, Box<dyn Error>> {
    let lines = line_iter.collect::<Vec<String>>();
    let mut count = 0;

    for ln in lines {
        let (test_value, operands) = ln.split_once(": ").unwrap();
        let test_value = test_value.parse::<u64>().unwrap();

        let operands: Vec<u64> = operands
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect_vec();

        fn branching_reducer(i: usize, operands: &[u64], accumulator: u64, target: u64) -> bool {
            if i == operands.len() - 1 {
                return accumulator == target;
            }

            branching_reducer(i + 1, operands, accumulator * operands[i + 1], target)
                || branching_reducer(i + 1, operands, accumulator + operands[i + 1], target)
        }

        if branching_reducer(0, &operands, operands[0], test_value) {
            count += test_value;
        }
    }

    Ok(count)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u64, Box<dyn Error>> {
    fn branching_reducer(i: usize, operands: &[u64], accumulator: u64, target: u64) -> bool {
        if i == operands.len() - 1 {
            return accumulator == target;
        }

        if accumulator > target {
            return false;
        }

        let added = accumulator + operands[i + 1];
        let multiplied = accumulator * operands[i + 1];
        let concat = format!("{}{}", accumulator, operands[i + 1])
            .parse::<u64>()
            .unwrap();

        [added, multiplied, concat]
            .iter()
            .any(|&accumulator| branching_reducer(i + 1, operands, accumulator, target))
    }

    let mut count = 0;

    for ln in line_iter {
        let (test_value, operands) = ln.split_once(": ").unwrap();
        let test_value = test_value.parse::<u64>().unwrap();

        let operands: Vec<u64> = operands
            .split(" ")
            .flat_map(|s| s.parse().ok())
            .collect_vec();

        if branching_reducer(0, &operands, operands[0], test_value) {
            count += test_value;
        }
    }

    Ok(count)
}
