use anyhow::{anyhow, Error, Result};
use std::{collections::HashSet, str::FromStr};

fn main() -> Result<()> {
    let input = include_str!("09.txt");
    println!(
        "Part 1: {}",
        number_of_positions_the_tail_visited(input, 2)?
    );
    println!(
        "Part 2: {}",
        number_of_positions_the_tail_visited(input, 10)?
    );
    Ok(())
}

fn number_of_positions_the_tail_visited(input: &str, knots: usize) -> Result<usize> {
    let mut map = Map::new(knots)?;
    for line in input.lines() {
        let instruction: Instruction = line.parse()?;
        map.execute(instruction);
    }
    Ok(map.tail_positions.len())
}

#[derive(Debug)]
struct Map {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Map {
    fn new(knots: usize) -> Result<Map> {
        if knots < 2 {
            return Err(anyhow!("need at least two knots: {}", knots));
        }
        let knots = vec![Position::default(); knots];
        Ok(Map {
            knots,
            tail_positions: HashSet::from_iter([Position::default()].into_iter()),
        })
    }

    fn execute(&mut self, instruction: Instruction) {
        for _ in 0..instruction.count {
            self.move_head(instruction.direction);
            self.move_tailing_knots();
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.knots[0].move_in_direction(direction);
    }

    fn move_tailing_knots(&mut self) {
        use Direction::*;
        for i in 0..(self.knots.len() - 1) {
            let first = self.knots[i];
            let second = self.knots.get_mut(i + 1).unwrap();
            let delta_x = first.x - second.x;
            let delta_y = first.y - second.y;
            if delta_x.abs() == 2 {
                if delta_x == 2 {
                    second.move_in_direction(Right);
                } else {
                    second.move_in_direction(Left)
                }
                if delta_y > 0 {
                    second.move_in_direction(Up)
                } else if delta_y < 0 {
                    second.move_in_direction(Down)
                }
            } else if delta_y.abs() == 2 {
                if delta_y == 2 {
                    second.move_in_direction(Up);
                } else {
                    second.move_in_direction(Down);
                }
                if delta_x > 0 {
                    second.move_in_direction(Right)
                } else if delta_x < 0 {
                    second.move_in_direction(Left)
                }
            }
        }
        self.tail_positions.insert(*self.knots.last().unwrap());
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");
        let direction = iter
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))
            .and_then(|s| s.parse::<Direction>())?;
        let count = iter
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))
            .and_then(|s| s.parse::<usize>().map_err(Error::from))?;
        Ok(Instruction { direction, count })
    }
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match s {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(anyhow!("unexpected direction: {}", s)),
        }
    }
}

impl Position {
    fn move_in_direction(&mut self, direction: Direction) {
        use Direction::*;
        match direction {
            Left => self.x -= 1,
            Right => self.x += 1,
            Up => self.y += 1,
            Down => self.y -= 1,
        }
    }
}

#[test]
fn part_1() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(number_of_positions_the_tail_visited(input, 2).unwrap(), 13);
}

#[test]
fn part_2() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(number_of_positions_the_tail_visited(input, 10).unwrap(), 1);

    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(number_of_positions_the_tail_visited(input, 10).unwrap(), 36);
}
