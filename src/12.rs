use anyhow::{anyhow, Error, Result};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() -> Result<()> {
    let input = include_str!("12.txt");
    println!("Part 1: {}", length_of_shortest_path(input)?);
    println!(
        "Part 2: {}",
        length_of_shortest_path_from_base_height(input)?
    );
    Ok(())
}

fn length_of_shortest_path(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    Ok(map.shortest_path()?.len() - 1)
}

fn length_of_shortest_path_from_base_height(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    let mut path_lengths = Vec::new();
    for (&position, &height) in map.map.iter() {
        if height == 0 {
            map.starting_position = position;
            if let Ok(path) = map.shortest_path() {
                path_lengths.push(path.len() - 1);
            }
        }
    }
    path_lengths
        .into_iter()
        .min()
        .ok_or_else(|| anyhow!("no paths found anywhere"))
}

#[derive(Debug)]
struct Map {
    map: HashMap<Position, i8>,
    starting_position: Position,
    ending_position: Position,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: i64,
    col: i64,
}

impl Map {
    fn shortest_path(&self) -> Result<Vec<Position>> {
        let mut paths = vec![vec![self.starting_position]];
        let mut new_paths = Vec::new();
        let mut seen = HashSet::new();
        while !paths.is_empty() {
            for path in paths.drain(..) {
                let position = path.last().expect("path should have an end");
                for neighbor in position.neighbors() {
                    if !seen.contains(&neighbor) && self.is_legal_move(*position, neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        if neighbor == self.ending_position {
                            return Ok(new_path);
                        } else {
                            new_paths.push(new_path);
                            seen.insert(neighbor);
                        }
                    }
                }
            }
            std::mem::swap(&mut new_paths, &mut paths);
        }
        Err(anyhow!("no paths found"))
    }

    fn is_legal_move(&self, from: Position, to: Position) -> bool {
        if let Some(to) = self.map.get(&to) {
            if let Some(from) = self.map.get(&from) {
                to - from <= 1
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Map> {
        let mut map = HashMap::new();
        let mut starting_position = None;
        let mut ending_position = None;
        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let position = Position {
                    row: row.try_into()?,
                    col: col.try_into()?,
                };
                if c == 'S' {
                    starting_position = Some(position);
                } else if c == 'E' {
                    ending_position = Some(position);
                }
                map.insert(position, height(c)?);
            }
        }
        Ok(Map {
            map,
            starting_position: starting_position.ok_or_else(|| anyhow!("no starting position"))?,
            ending_position: ending_position.ok_or_else(|| anyhow!("no ending position"))?,
        })
    }
}

impl Position {
    fn neighbors(&self) -> [Position; 4] {
        [
            Position {
                row: self.row - 1,
                col: self.col,
            },
            Position {
                row: self.row + 1,
                col: self.col,
            },
            Position {
                row: self.row,
                col: self.col - 1,
            },
            Position {
                row: self.row,
                col: self.col + 1,
            },
        ]
    }
}

fn height(mut c: char) -> Result<i8> {
    if c == 'S' {
        return Ok(0);
    } else if c == 'E' {
        c = 'z'
    }
    (u32::from(c) - u32::from('a'))
        .try_into()
        .map_err(Error::from)
}

#[test]
fn part_1() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(length_of_shortest_path(input).unwrap(), 31);
}

#[test]
fn part_2() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(length_of_shortest_path_from_base_height(input).unwrap(), 29);
}
