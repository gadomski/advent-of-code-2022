use anyhow::{anyhow, Error};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let input = include_str!("02.txt");
    println!("Part 1: {}", total_score(input, false)?);
    println!("Part 2: {}", total_score(input, true)?);
    Ok(())
}

fn total_score(input: &str, second_value_is_result: bool) -> Result<i64, Error> {
    let game = Game {
        second_value_is_result,
    };
    game.total_score(input)
}

#[derive(Debug)]
struct Game {
    second_value_is_result: bool,
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum WinLossDraw {
    Win,
    Loss,
    Draw,
}

impl Game {
    fn total_score(&self, input: &str) -> Result<i64, Error> {
        let mut score = 0;
        for line in input.lines() {
            score += self.round_score(line)?;
        }
        Ok(score)
    }

    fn round_score(&self, input: &str) -> Result<i64, Error> {
        let mut iter = input.split(" ");
        let other = iter
            .next()
            .ok_or_else(|| anyhow!("invalid round: {}", input))
            .and_then(|s| s.parse::<Shape>())?;
        let (win_loss_draw, me) = if self.second_value_is_result {
            let win_loss_draw = iter
                .next()
                .ok_or_else(|| anyhow!("invalid round: {}", input))
                .and_then(|s| s.parse::<WinLossDraw>())?;
            let me = win_loss_draw.me(&other);
            (win_loss_draw, me)
        } else {
            let me = iter
                .next()
                .ok_or_else(|| anyhow!("invalid round: {}", input))
                .and_then(|s| s.parse::<Shape>())?;
            (me.win_loss_draw(&other), me)
        };
        Ok(me.score() + win_loss_draw.score())
    }
}

impl Shape {
    fn score(&self) -> i64 {
        use Shape::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn win_loss_draw(&self, other: &Shape) -> WinLossDraw {
        use Shape::*;
        use WinLossDraw::*;
        match self {
            Rock => match other {
                Rock => Draw,
                Paper => Loss,
                Scissors => Win,
            },
            Paper => match other {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss,
            },
            Scissors => match other {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw,
            },
        }
    }
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Shape::*;
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(anyhow!("invalid shape: {}", s)),
        }
    }
}

impl WinLossDraw {
    fn score(&self) -> i64 {
        use WinLossDraw::*;
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }

    fn me(&self, other: &Shape) -> Shape {
        use Shape::*;
        use WinLossDraw::*;
        match self {
            Win => match other {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Loss => match other {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Draw => match other {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
        }
    }
}

impl FromStr for WinLossDraw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use WinLossDraw::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(anyhow!("invalid win loss draw: {}", s)),
        }
    }
}

#[test]
fn part_1() {
    let game = Game {
        second_value_is_result: false,
    };
    assert_eq!(game.round_score("A Y").unwrap(), 8);
    assert_eq!(game.round_score("B X").unwrap(), 1);
    assert_eq!(game.round_score("C Z").unwrap(), 6);
    assert_eq!(game.total_score("A Y\nB X\nC Z").unwrap(), 15);
}

#[test]
fn part_2() {
    let game = Game {
        second_value_is_result: true,
    };
    assert_eq!(game.round_score("A Y").unwrap(), 4);
    assert_eq!(game.round_score("B X").unwrap(), 1);
    assert_eq!(game.round_score("C Z").unwrap(), 7);
    assert_eq!(game.total_score("A Y\nB X\nC Z").unwrap(), 12);
}
