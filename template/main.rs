use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::aoc_test;

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    // let grid = line_iter.map(|l| l.bytes().collect_vec()).collect_vec();
    // let lines = line_iter.collect::<Vec<String>>();

    Ok(0)
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
        let input = utils::file_line_iter("./text/%%DAY%%");
        assert_eq!(solution1(input).unwrap(), 0);
    }

    #[test]
    fn test_solution2() {
        let input = utils::file_line_iter("./text/%%DAY%%");
        assert_eq!(solution2(input).unwrap(), 0);
    }
}
