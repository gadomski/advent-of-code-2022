use anyhow::{anyhow, Error};
use std::{collections::HashMap, str::FromStr};

fn main() -> Result<(), Error> {
    let input = include_str!("05.txt");
    println!("Part 1: {}", top_of_stacks(input, false)?);
    println!("Part 2: {}", top_of_stacks(input, true)?);
    Ok(())
}

fn top_of_stacks(input: &str, retain_order: bool) -> Result<String, Error> {
    let (front, back) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input, no double newline: {}", input))?;
    let mut stacks: Stacks = front.parse()?;
    for line in back.lines() {
        let instruction: Instruction = line.parse()?;
        stacks.execute(&instruction, retain_order)?;
    }
    let mut stack_names: Vec<_> = stacks.0.keys().collect();
    stack_names.sort();
    let mut top_of_stacks = String::new();
    for stack_name in stack_names {
        let crate_name = stacks
            .stack(*stack_name)?
            .last()
            .ok_or_else(|| anyhow!("empty stack: {}", stack_name))?;
        top_of_stacks.push(*crate_name);
    }
    Ok(top_of_stacks)
}

#[derive(Debug)]
struct Stacks(HashMap<char, Vec<char>>);

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: char,
    to: char,
}

impl Stacks {
    fn execute(&mut self, instruction: &Instruction, retain_order: bool) -> Result<(), Error> {
        if retain_order {
            self.move_crates(instruction.from, instruction.to, instruction.count)?;
        } else {
            for _ in 0..instruction.count {
                self.move_crate(instruction.from, instruction.to)?;
            }
        }
        Ok(())
    }

    fn move_crate(&mut self, from: char, to: char) -> Result<(), Error> {
        let crate_name = {
            let from_stack = self.stack_mut(from)?;
            if let Some(crate_name) = from_stack.pop() {
                crate_name
            } else {
                return Err(anyhow!("empty stack: {}", from));
            }
        };
        let to_stack = self.stack_mut(to)?;
        to_stack.push(crate_name);
        Ok(())
    }

    fn move_crates(&mut self, from: char, to: char, count: usize) -> Result<(), Error> {
        let crates = {
            let from_stack = self.stack_mut(from)?;
            let mut crates = vec![];
            for _ in 0..count {
                if let Some(crate_name) = from_stack.pop() {
                    crates.insert(0, crate_name);
                } else {
                    return Err(anyhow!("empty stack: {}", from));
                }
            }
            crates
        };
        let to_stack = self.stack_mut(to)?;
        for crate_name in crates {
            to_stack.push(crate_name);
        }
        Ok(())
    }

    fn stack(&self, name: char) -> Result<&Vec<char>, Error> {
        self.0
            .get(&name)
            .ok_or_else(|| anyhow!("invalid stack: {}", name))
    }

    fn stack_mut(&mut self, name: char) -> Result<&mut Vec<char>, Error> {
        self.0
            .get_mut(&name)
            .ok_or_else(|| anyhow!("invalid stack: {}", name))
    }
}

impl FromStr for Stacks {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut rows = vec![];
        while let Some(line) = lines.next() {
            if lines.peek().is_some() {
                let mut row = vec![];
                for i in (0..line.len()).step_by(4) {
                    let s = if i < line.len() - 4 {
                        &line[i..i + 4]
                    } else {
                        &line[i..]
                    };
                    if s.chars().all(|c| c == ' ') {
                        row.push(None);
                    } else {
                        row.push(Some(parse_crate_name(s)?));
                    }
                }
                rows.push(row);
            } else {
                let mut stacks = HashMap::new();
                for stack_name in line.split_ascii_whitespace() {
                    let stack_name = stack_name.chars().next().unwrap();
                    let mut stack = vec![];
                    for row in rows.iter_mut().rev() {
                        if row.is_empty() {
                            return Err(anyhow!("unexpectedly empty row"));
                        } else if let Some(crate_name) = row.remove(0) {
                            stack.push(crate_name)
                        }
                    }
                    stacks.insert(stack_name, stack);
                }
                return Ok(Stacks(stacks));
            }
        }
        unreachable!()
    }
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        expect_word(&mut words, "move")?;
        let count: usize = parse_word(&mut words)?;
        expect_word(&mut words, "from")?;
        let from: char = parse_word(&mut words)?;
        expect_word(&mut words, "to")?;
        let to: char = parse_word(&mut words)?;
        Ok(Instruction { count, from, to })
    }
}

fn parse_crate_name(s: &str) -> Result<char, Error> {
    let mut chars = s.chars();
    if let Some(start) = chars.next() {
        if start == '[' {
            if let Some(crate_name) = chars.next() {
                if let Some(end) = chars.next() {
                    if end == ']' {
                        Ok(crate_name)
                    } else {
                        Err(anyhow!("crate does not end in ']': {}", s))
                    }
                } else {
                    Err(anyhow!("crate too short: {}", s))
                }
            } else {
                Err(anyhow!("crate too short: {}", s))
            }
        } else {
            Err(anyhow!("crate too short: {}", s))
        }
    } else {
        Err(anyhow!("crate empty: {}", s))
    }
}

fn expect_word<'a>(mut iter: impl Iterator<Item = &'a str>, word: &str) -> Result<(), Error> {
    if let Some(s) = iter.next() {
        if s == word {
            Ok(())
        } else {
            Err(anyhow!("expected {}, got {}", word, s))
        }
    } else {
        Err(anyhow!("expected {}, got end of iterator", word))
    }
}

fn parse_word<'a, T>(mut iter: impl Iterator<Item = &'a str>) -> Result<T, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + std::error::Error + 'static,
{
    if let Some(s) = iter.next() {
        s.parse().map_err(Error::from)
    } else {
        Err(anyhow!("expected parseable value, got end of iterator",))
    }
}

#[test]
fn part_1() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(top_of_stacks(input, false).unwrap(), "CMZ");
}

#[test]
fn part_2() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(top_of_stacks(input, true).unwrap(), "MCD");
}
