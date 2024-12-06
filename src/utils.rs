use std::env;
use std::path::Path;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

pub fn parse_lines() -> Result<Vec<String>, Box<dyn Error>> {
    std::io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
}

pub fn parse_line() -> Result<String, Box<dyn Error>> {
    Ok(parse_lines()?.join("\n"))
}

pub fn parse_file_lines(filename: impl AsRef<Path>) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filename.as_ref())?;
    Ok(contents.lines().map(|s| s.to_string()).collect())
}

pub fn file_line_iter(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    reader.lines().map(|l| l.expect("Failed to read line"))
}

#[macro_export]
macro_rules! aoc_test {
    (day: $day:tt, version: $version:tt, part1: $part1:expr, part2: $part2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::utils;

            const TEST_INPUT_FILE: &str = concat!("./text/", stringify!($day));

            #[test]
            fn test_solution1() {
                let input = utils::file_line_iter(TEST_INPUT_FILE);
                assert_eq!(solution1(input).unwrap(), $part1);
            }

            #[test]
            fn test_solution2() {
                let input = utils::file_line_iter(TEST_INPUT_FILE);
                assert_eq!(solution2(input).unwrap(), $part2);
            }
        }
    };
}
