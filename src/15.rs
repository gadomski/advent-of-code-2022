use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, ops::RangeInclusive};

fn main() -> Result<()> {
    let input = include_str!("15.txt");
    let map = Map::new(input, 4_000_000)?;
    println!(
        "Part 1: {}",
        map.number_of_positions_without_beacon_in_row(2_000_000)
    );
    println!(
        "Part 2: {}",
        map.distress_beacon_tuning_frequency(4_000_000)?
    );
    Ok(())
}

#[derive(Debug)]
struct Map {
    rows: HashMap<i64, Row>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Default)]
struct Row(Vec<RangeInclusive<i64>>);

impl Map {
    fn new(input: &str, max_y: i64) -> Result<Map> {
        let mut rows: HashMap<i64, Row> = HashMap::new();
        for line in input.lines() {
            let mut iter = line.split_ascii_whitespace();
            expect_word(&mut iter, "Sensor")?;
            expect_word(&mut iter, "at")?;
            let sensor_x = next_as_i64(&mut iter, "x", ',')?;
            let sensor_y = next_as_i64(&mut iter, "y", ':')?;
            let sensor = Position::from((sensor_x, sensor_y));
            expect_word(&mut iter, "closest")?;
            expect_word(&mut iter, "beacon")?;
            expect_word(&mut iter, "is")?;
            expect_word(&mut iter, "at")?;
            let beacon_x = next_as_i64(&mut iter, "x", ',')?;
            let beacon_y = next_as_i64(&mut iter, "y", None)?;
            let beacon = Position::from((beacon_x, beacon_y));
            for (row, range) in sensor.ranges_at_least_as_close_as(beacon, max_y) {
                let entry = rows.entry(row).or_default();
                entry.add(range);
            }
        }
        Ok(Map { rows })
    }

    fn number_of_positions_without_beacon_in_row(&self, row: i64) -> i64 {
        if let Some(row) = self.rows.get(&row) {
            let mut count = 0;
            for range in &row.0 {
                count += range.end() - range.start();
            }
            count
        } else {
            0
        }
    }

    fn distress_beacon_tuning_frequency(&self, max_coordinate: i64) -> Result<i64> {
        for i in 0..=max_coordinate {
            if let Some(row) = self.rows.get(&i) {
                for range in &row.0 {
                    if *range.end() >= 0 && *range.end() <= max_coordinate {
                        return Ok((range.end() + 1) * 4_000_000 + i);
                    }
                }
            }
        }
        Err(anyhow!("no row found without a beacon"))
    }
}

impl Row {
    fn add(&mut self, mut range: RangeInclusive<i64>) {
        for old in std::mem::take(&mut self.0) {
            if old.end() < range.start() || range.end() < old.start() {
                self.0.push(old);
            } else {
                range = (std::cmp::min(*range.start(), *old.start()))
                    ..=(std::cmp::max(*range.end(), *old.end()));
            }
        }
        self.0.push(range);
    }
}

impl Position {
    fn ranges_at_least_as_close_as(
        &self,
        other: Position,
        max_y: i64,
    ) -> Vec<(i64, RangeInclusive<i64>)> {
        let distance = self.manhattan_distance(other);
        let y_min = std::cmp::max(self.y - distance, 0);
        let y_max = std::cmp::min(self.y + distance, max_y);
        let mut ranges = Vec::new();
        for y in y_min..=y_max {
            let remainder = distance - (self.y - y).abs();
            let x_min = self.x - remainder;
            let x_max = self.x + remainder;
            ranges.push((y, x_min..=x_max));
        }
        ranges
    }

    fn manhattan_distance(&self, other: Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position { x, y }
    }
}

fn expect_word<'a>(iter: impl Iterator<Item = &'a str>, word: &str) -> Result<()> {
    next(iter).and_then(|s| {
        if s == word {
            Ok(())
        } else {
            Err(anyhow!("unexpected word, expected={}, actual={}", word, s))
        }
    })
}

fn next<'a>(mut iter: impl Iterator<Item = &'a str>) -> Result<&'a str> {
    iter.next()
        .ok_or_else(|| anyhow!("unexpected end of input"))
}

fn next_as_i64<'a>(
    iter: impl Iterator<Item = &'a str>,
    name: &str,
    trailing_char: impl Into<Option<char>>,
) -> Result<i64> {
    let s = next(iter)?;
    let mut iter = s.split('=');
    expect_word(&mut iter, name)?;
    let s = next(iter)?;
    if let Some(trailing_char) = trailing_char.into() {
        if !s.ends_with(trailing_char) {
            return Err(anyhow!("{}=n, should end in a comma: {}", name, s));
        } else {
            s[0..(s.len() - 1)].parse().map_err(Error::from)
        }
    } else {
        s.parse().map_err(Error::from)
    }
}

#[test]
fn part_1() {
    let map = Map::new(test_input(), 10).unwrap();
    assert_eq!(map.number_of_positions_without_beacon_in_row(10), 26);
}

#[test]
fn part_2() {
    let map: Map = Map::new(test_input(), 20).unwrap();
    assert_eq!(map.distress_beacon_tuning_frequency(20).unwrap(), 56000011);
}

#[cfg(test)]
fn test_input() -> &'static str {
    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
}
