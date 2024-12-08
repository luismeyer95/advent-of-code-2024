use itertools::{chain, Itertools};
use regex::Regex;
use std::error::Error;

use crate::aoc_test;

aoc_test!(day: d3, version: main, part1: 190604937, part2: 82857512);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let text = line_iter.collect::<Vec<String>>().join("");
    let captures: Vec<_> = re.captures_iter(&text).collect();
    let mut result = 0;
    for cap in captures {
        result += cap.get(1).unwrap().as_str().parse::<u32>().unwrap()
            * cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
    }

    Ok(result)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let text = line_iter.collect::<Vec<String>>().join("");
    let cleaned_text = Regex::new(r"don't\(\).*?(do\(\)|$)")?.replace_all(&text, "");

    let result = Regex::new(r"mul\((\d+),(\d+)\)")?
        .captures_iter(&cleaned_text)
        .map(|cap| {
            let (left, right) = (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
            left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
        })
        .sum();

    Ok(result)
}
//
// pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
// }
