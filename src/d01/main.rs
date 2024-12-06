use crate::aoc_test;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;

aoc_test!(day: d01, version: main, part1: 2769675, part2: 24643097);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let lines = line_iter.collect::<Vec<String>>();

    // for each line
    // - split whitespace and map to pair of nums
    // - left is added to left heap, right to right heap
    // zip iter over the heaps, sum the diffs
    // return result

    let mut left_heap = BinaryHeap::<u32>::new();
    let mut right_heap = BinaryHeap::<u32>::new();

    for line in lines {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<u32>()?;
        let right = right.parse::<u32>()?;

        left_heap.push(left);
        right_heap.push(right);
    }

    let mut sum = 0;
    while let Some(left) = left_heap.pop() {
        if let Some(right) = right_heap.pop() {
            sum += left.abs_diff(right);
        }
    }

    Ok(sum)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    let lines = line_iter.collect::<Vec<String>>();

    // for each line
    // - split whitespace and map to pair of nums
    // - add left to list, add right to counter dict
    // for each n in left, add n * r[n] to result
    // return result

    let mut leftc: Vec<u32> = vec![];
    let mut rightc: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once("   ").unwrap();
        let left = left.parse::<u32>()?;
        let right = right.parse::<u32>()?;

        leftc.push(left);
        rightc.entry(right).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut sum = 0;
    for n in leftc {
        sum += n * rightc.get(&n).unwrap_or(&0);
    }

    Ok(sum)
}
