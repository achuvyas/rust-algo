use std::io::Result;
use std::str::FromStr;
use itertools::Itertools;

fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn max_calorie() -> Result<usize> {
    Ok(read_one_per_line::<u32>("./data/aoc202201.txt")?.
}
