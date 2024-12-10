use itertools::{zip, Itertools};
use nom::AsBytes;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    iter::repeat,
};

use crate::aoc_test;

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let mut line: Vec<i32> = line_iter
        .flat_map(|s| s.bytes().collect_vec())
        .map(|d| d - b'0')
        .enumerate()
        .flat_map(|(i, d)| {
            let id = if i % 2 == 0 { i as i32 / 2 } else { -1 };
            repeat(id).take(d.into())
        })
        .collect();

    let (mut l, mut r) = (0, line.len() - 1);

    while l < r {
        match () {
            _ if line[l] != -1 => l += 1,
            _ if line[r] == -1 => r -= 1,
            _ => line.swap(l, r),
        }
    }

    let checksum = line
        .into_iter()
        .take_while(|&n| n != -1)
        .enumerate()
        .fold(0, |acc, (i, d)| acc + i * d as usize);

    Ok(checksum)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    #[derive(Debug, Clone)]
    struct Block(u64 /* size */, u64 /* occupied */);

    // checksum? multiply id of block w/ flat index
    // id? index of block / 2 in compact repr
    // flat index? index of block in flat repr
    // checksum(block) =
    // compact_repr(block index / 2) * flat_repr(block index.. block index + size)
    fn block_checksum(flat_index: u64, block_index: usize, b: Block) -> usize {
        (0..b.0)
            .map(|i| (flat_index + b.1 + i) as usize * (block_index / 2))
            .sum::<usize>()
    }

    let mut checksum: usize = 0;
    let mut ln: Vec<Block> = line_iter
        .flat_map(|l| l.bytes().collect_vec())
        .map(|d| Block((d - b'0').into(), 0))
        .collect::<_>();

    for r in (0..ln.len()).rev().filter(|&r| r % 2 == 0) {
        let Block(moving_block_size, _) = ln[r];
        let (mut l, mut flat_index) = (0, 0);
        // Find a leftward free span that's big enough
        while l < r {
            let Block(size, occupied) = ln[l];
            if l % 2 != 0 && size - occupied >= moving_block_size {
                break;
            }
            flat_index += size;
            l += 1;
        }

        let Block(_, occupied) = ln[l];
        if l < r {
            // Found it, swap the block into the free span
            checksum += block_checksum(flat_index, r, Block(moving_block_size, occupied));
            ln[l].1 += moving_block_size;
        } else {
            // Can't move block, add unmoved block checksum
            checksum += block_checksum(flat_index, l, Block(moving_block_size, 0));
        }
    }

    Ok(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn test_solution2() {
        // 23331131402
        // 23....    0 { 1: 2 }, chk += (2 + 0 + 0) * 5 + (2 + 1 + 0) * 5 = 25
        // 233...  400 cannot move 4
        // 2333  01400 { 3: 3 }, chk += (8 + 0 + 0) * 3 + (8 + 1 + 0) * 3 + (8 + 2 + 0) * 3 = 81
        // 23330101400 { 1: 3(+1) }, chk += (2 + 0 + 2) * 2 = 8
        // chk += (0*0+1*0)+(5*1+6*1+7*1)+0*(11*2) ... + (13*4+14*4+15*4+16*4) = 18 + 232 = 250
        // chk = 364

        // actual: chk

        let s = vec!["23331131402".to_owned()];
        assert_eq!(solution2(s.into_iter()).unwrap(), 428);
    }
}
