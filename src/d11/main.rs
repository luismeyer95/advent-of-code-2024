use cached::proc_macro::cached;
use std::error::Error;

pub fn solution1(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let line = line_iter.collect::<Vec<String>>().join("");
    let stones: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let count = stones.iter().map(|s| count_stones(*s, 25)).sum::<u64>();

    Ok(count as usize)
}

pub fn solution2(line_iter: impl Iterator<Item = String>) -> Result<usize, Box<dyn Error>> {
    let line = line_iter.collect::<Vec<String>>().join("");
    let stones: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let count = stones.iter().map(|s| count_stones(*s, 75)).sum::<u64>();

    Ok(count as usize)
}

#[cached]
pub fn count_stones(stone: u64, iteration: u64) -> u64 {
    if iteration == 0 {
        return 1;
    }

    let digit_count = stone.ilog10() + 1;

    match stone {
        0 => count_stones(1, iteration - 1),
        _ if digit_count & 1 == 0 => {
            let sep = 10u64.pow(digit_count / 2);
            count_stones(stone % sep, iteration - 1) + count_stones(stone / sep, iteration - 1)
        }
        _ => count_stones(stone * 2024, iteration - 1),
    }
}
