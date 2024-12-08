use crate::aoc_test;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::iter::zip;

aoc_test!(day: d1, version: alt, part1: 2769675, part2: 24643097);

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    line_iter
        .filter_map(|line| {
            line.split_once("   ")
                .and_then(|(l, r)| (l.parse::<u32>().ok()?, r.parse().ok()?).into())
        })
        .try_fold(
            (BinaryHeap::new(), BinaryHeap::new()),
            |(mut lh, mut rh), (l, r)| {
                lh.push(l);
                rh.push(r);
                Ok((lh, rh))
            },
        )
        .map(|(mut lh, mut rh)| {
            zip(lh.drain_sorted(), rh.drain_sorted())
                .map(|(l, r)| l.abs_diff(r))
                .sum()
        })
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<u32, Box<dyn Error>> {
    line_iter
        .filter_map(|line| {
            line.split_once("   ")
                .and_then(|(l, r)| (l.parse::<u32>().ok()?, r.parse::<u32>().ok()?).into())
        })
        .try_fold((Vec::new(), HashMap::new()), |(mut lh, mut rh), (l, r)| {
            lh.push(l);
            rh.entry(r).and_modify(|c| *c += 1).or_insert(1);
            Ok((lh, rh))
        })
        .map(|(lh, rh)| lh.iter().map(|l| l * rh.get(l).unwrap_or(&0)).sum())
}
