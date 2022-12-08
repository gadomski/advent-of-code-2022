use anyhow::{anyhow, Error};
use std::{collections::HashMap, str::FromStr};

fn main() -> Result<(), Error> {
    let forest: Forest = include_str!("08.txt").parse().unwrap();
    println!("Part 1: {}", forest.number_of_visible_trees());
    println!("Part 2: {}", forest.highest_scenic_score());
    Ok(())
}

#[derive(Debug)]
struct Forest {
    trees: HashMap<(usize, usize), u32>,
    height: usize,
    width: usize,
}

impl Forest {
    fn number_of_visible_trees(&self) -> u64 {
        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.is_visible(row, col) {
                    count += 1;
                }
            }
        }
        count
    }

    fn scenic_score(&self, row: usize, col: usize) -> u64 {
        let height = self.trees[&(row, col)];
        self.count_visible_trees(height, (0..row).rev().map(|r| (r, col)))
            * self.count_visible_trees(height, ((row + 1)..self.height).map(|r| (r, col)))
            * self.count_visible_trees(height, (0..col).rev().map(|c| (row, c)))
            * self.count_visible_trees(height, ((col + 1)..self.width).map(|c| (row, c)))
    }

    fn count_visible_trees(&self, height: u32, iter: impl Iterator<Item = (usize, usize)>) -> u64 {
        let mut count = 0;
        for (row, col) in iter {
            count += 1;
            if self.trees[&(row, col)] >= height {
                return count;
            }
        }
        count
    }

    fn highest_scenic_score(&self) -> u64 {
        let mut max = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let scenic_score = self.scenic_score(row, col);
                if scenic_score > max {
                    max = scenic_score;
                }
            }
        }
        max
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        self.is_visible_from_top(row, col)
            || self.is_visible_from_right(row, col)
            || self.is_visible_from_bottom(row, col)
            || self.is_visible_from_left(row, col)
    }

    fn is_visible_from_top(&self, row: usize, col: usize) -> bool {
        let height = self.trees[&(row, col)];
        (0..row).all(|r| height > self.trees[&(r, col)])
    }

    fn is_visible_from_bottom(&self, row: usize, col: usize) -> bool {
        let height = self.trees[&(row, col)];
        ((row + 1)..self.height).all(|r| height > self.trees[&(r, col)])
    }

    fn is_visible_from_left(&self, row: usize, col: usize) -> bool {
        let height = self.trees[&(row, col)];
        (0..col).all(|c| height > self.trees[&(row, c)])
    }

    fn is_visible_from_right(&self, row: usize, col: usize) -> bool {
        let height = self.trees[&(row, col)];
        ((col + 1)..self.width).all(|c| height > self.trees[&(row, c)])
    }
}

impl FromStr for Forest {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.trim().lines().collect();
        if lines.is_empty() {
            return Err(anyhow!("empty input"));
        }
        let height = lines.len();
        let width = lines[0].len();
        let mut trees = HashMap::new();
        for (row, line) in lines.into_iter().enumerate() {
            if line.len() != width {
                return Err(anyhow!("irregular widths"));
            }
            for (col, c) in line.chars().enumerate() {
                trees.insert(
                    (row, col),
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("could not convert char to height: {}", c))?,
                );
            }
        }
        Ok(Forest {
            trees,
            height,
            width,
        })
    }
}

#[test]
fn part_1() {
    let input = "30373
25512
65332
33549
35390";
    let forest: Forest = input.parse().unwrap();
    assert_eq!(forest.number_of_visible_trees(), 21);
}

#[test]
fn part_2() {
    let input = "30373
25512
65332
33549
35390";
    let forest: Forest = input.parse().unwrap();
    assert_eq!(forest.scenic_score(1, 2), 4);
    assert_eq!(forest.scenic_score(3, 2), 8);
    assert_eq!(forest.highest_scenic_score(), 8);
}
