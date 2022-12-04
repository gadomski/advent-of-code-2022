use anyhow::{anyhow, Error};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("04.txt");
    println!("Part 1: {}", number_of_redundant_pairs(input)?);
    println!("Part 2: {}", number_of_overlapping_pairs(input)?);
    Ok(())
}

fn number_of_redundant_pairs(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        let ranges = line
            .split(',')
            .map(|s| s.parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?;
        if ranges.len() != 2 {
            return Err(anyhow!("invalid input line: {}", line));
        }
        if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]) {
            count += 1;
        }
    }
    Ok(count)
}

fn number_of_overlapping_pairs(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        let ranges = line
            .split(',')
            .map(|s| s.parse::<Range>())
            .collect::<Result<Vec<_>, _>>()?;
        if ranges.len() != 2 {
            return Err(anyhow!("invalid input line: {}", line));
        }
        if ranges[0].overlaps(&ranges[1]) {
            count += 1;
        }
    }
    Ok(count)
}

#[derive(Debug)]
struct Range(usize, usize);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        !(self.1 < other.0 || self.0 > other.1)
    }
}

impl FromStr for Range {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split('-')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        if values.len() != 2 {
            Err(anyhow!("invalid range: {}", s))
        } else {
            Ok(Range(values[0], values[1]))
        }
    }
}

#[test]
fn part_1() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(number_of_redundant_pairs(input).unwrap(), 2);
}

#[test]
fn part_2() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(number_of_overlapping_pairs(input).unwrap(), 4);
}
