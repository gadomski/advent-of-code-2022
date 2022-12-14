use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, fmt::Display, str::FromStr};

const STARTING_POSITION: Position = Position { x: 500, y: 0 };

fn main() -> Result<()> {
    let input = include_str!("14.txt");
    let mut cave: Cave = input.parse()?;
    cave.simulate(false);
    println!("Part 1: {}", cave.iter_sand().count());
    let mut cave: Cave = input.parse()?;
    cave.simulate(true);
    println!("Part 2: {}", cave.iter_sand().count());
    Ok(())
}

#[derive(Debug)]
struct Cave {
    positions: HashMap<Position, bool>,
    min_x: i64,
    max_x: i64,
    max_y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Cave {
    fn simulate(&mut self, with_floor: bool) {
        let mut sand = STARTING_POSITION;
        loop {
            if self.positions.contains_key(&STARTING_POSITION) {
                break;
            }
            sand = self.simulate_one(sand);
            if with_floor {
                if sand.y >= self.max_y + 1 {
                    sand = self.insert_sand(sand);
                }
            } else if sand.y > self.max_y {
                break;
            }
        }
    }

    fn simulate_one(&mut self, sand: Position) -> Position {
        let below = Position {
            x: sand.x,
            y: sand.y + 1,
        };
        if !self.positions.contains_key(&below) {
            return below;
        }
        let down_left = Position {
            x: sand.x - 1,
            y: sand.y + 1,
        };
        if !self.positions.contains_key(&down_left) {
            return down_left;
        }
        let down_right = Position {
            x: sand.x + 1,
            y: sand.y + 1,
        };
        if !self.positions.contains_key(&down_right) {
            return down_right;
        }
        self.insert_sand(sand)
    }

    fn insert_sand(&mut self, sand: Position) -> Position {
        if sand.x < self.min_x {
            self.min_x = sand.x;
        }
        if sand.x > self.max_x {
            self.max_x = sand.x;
        }
        self.positions.insert(sand, true);
        STARTING_POSITION
    }

    fn iter_sand(&self) -> impl Iterator<Item = &Position> {
        self.positions
            .iter()
            .filter_map(|(position, &is_sand)| if is_sand { Some(position) } else { None })
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cave> {
        let mut map = HashMap::new();
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for line in s.lines() {
            let positions = line
                .split(" -> ")
                .map(|s| s.parse::<Position>())
                .collect::<Result<Vec<_>>>()?;
            for (start, end) in positions.iter().zip(positions.iter().skip(1)) {
                for position in start.line_to(end)? {
                    if position.x < min_x {
                        min_x = position.x;
                    }
                    if position.x > max_x {
                        max_x = position.x;
                    }
                    if position.y > max_y {
                        max_y = position.y;
                    }
                    map.insert(position, false);
                }
            }
        }
        Ok(Cave {
            positions: map,
            min_x,
            max_x,
            max_y,
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=(self.max_y + 2) {
            for x in self.min_x..=self.max_x {
                let position = Position { x, y };
                if let Some(&is_sand) = self.positions.get(&position) {
                    if is_sand {
                        write!(f, "o")?;
                    } else {
                        write!(f, "#")?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Position {
    fn line_to(&self, other: &Position) -> Result<Vec<Position>> {
        let mut line = Vec::new();
        if self.y == other.y {
            let (start, end) = if self.x > other.x {
                (other.x, self.x)
            } else {
                (self.x, other.x)
            };
            for x in start..(end + 1) {
                line.push(Position { x, y: self.y });
            }
        } else if self.x == other.x {
            let (start, end) = if self.y > other.y {
                (other.y, self.y)
            } else {
                (self.y, other.y)
            };
            for y in start..(end + 1) {
                line.push(Position { x: self.x, y });
            }
        } else {
            return Err(anyhow!("diagonal line: {:?} -> {:?}", self, other));
        }
        Ok(line)
    }
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Position> {
        let mut iter = s.split(',');
        let x = iter
            .next()
            .ok_or_else(|| anyhow!("no x coordinate"))
            .and_then(|s| s.parse::<i64>().map_err(Error::from))?;
        let y = iter
            .next()
            .ok_or_else(|| anyhow!("no y coordinate"))
            .and_then(|s| s.parse::<i64>().map_err(Error::from))?;
        Ok(Position { x, y })
    }
}

#[test]
fn part_1() {
    let mut cave: Cave = test_input().parse().unwrap();
    cave.simulate(false);
    assert_eq!(cave.iter_sand().count(), 24);
}

#[test]
fn part_2() {
    let mut cave: Cave = test_input().parse().unwrap();
    cave.simulate(true);
    assert_eq!(cave.iter_sand().count(), 93);
}

#[cfg(test)]
fn test_input() -> &'static str {
    "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
}
