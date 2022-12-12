use anyhow::{anyhow, Error, Result};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

fn main() -> Result<()> {
    let input = include_str!("10.txt");
    println!("Part 1: {}", sum_of_signal_strengths(input, 20, 40)?);

    let mut computer = Computer::with_input(input)?;
    computer.run()?;
    println!("Part 2:\n{}", computer);
    Ok(())
}

fn sum_of_signal_strengths(input: &str, start: u32, stride: u32) -> Result<i64> {
    let mut computer = Computer::with_input(input)?;
    let mut cycle = 0;
    let mut sum = 0;
    loop {
        cycle += 1;
        let value = computer.tick()?;
        if (cycle + start) % stride == 0 {
            sum += value * i64::from(cycle);
        }
        if computer.is_out_of_instructions() {
            break;
        }
    }
    Ok(sum)
}

#[derive(Debug)]
struct Computer {
    currently_executing: Option<(Instruction, u64)>,
    instructions: VecDeque<Instruction>,
    registers: HashMap<String, i64>,
    screen: [[bool; 40]; 6],
    pixel_row: usize,
    pixel_col: usize,
}

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Computer {
    fn with_input(input: &str) -> Result<Computer> {
        let mut computer = Computer::new();
        for line in input.lines() {
            let instruction = line.parse()?;
            computer.add_instruction(instruction);
        }
        Ok(computer)
    }

    fn new() -> Computer {
        let mut registers = HashMap::new();
        registers.insert("X".to_string(), 1);
        Computer {
            currently_executing: None,
            instructions: VecDeque::new(),
            registers,
            screen: [[false; 40]; 6],
            pixel_row: 0,
            pixel_col: 0,
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push_back(instruction);
    }

    fn run(&mut self) -> Result<()> {
        while !self.is_out_of_instructions() {
            let _ = self.tick()?;
        }
        Ok(())
    }

    fn draw(&mut self) {
        self.screen[self.pixel_row][self.pixel_col] = true;
    }

    fn is_lit(&self, row: usize, col: usize) -> bool {
        self.screen[row][col]
    }

    fn tick(&mut self) -> Result<i64> {
        let (instruction, cycles_left) =
            if let Some((currently_executing, cycles_left)) = self.currently_executing.take() {
                (currently_executing, cycles_left)
            } else {
                if let Some(next_instruction) = self.instructions.pop_front() {
                    let cycles_left = next_instruction.cycles_to_complete();
                    (next_instruction, cycles_left)
                } else {
                    return Err(anyhow!("unexpectedly out of instructions"));
                }
            };
        let sprite_position = self.sprite_position();
        if (sprite_position - i64::try_from(self.pixel_col)?).abs() <= 1 {
            self.draw();
        }
        self.pixel_col += 1;
        if self.pixel_col >= self.screen[0].len() {
            self.pixel_col = 0;
            self.pixel_row += 1;
        }
        if self.pixel_row >= self.screen.len() {
            self.pixel_col = 0;
            self.pixel_row = 0;
        }
        if cycles_left == 1 {
            self.execute(instruction);
        } else {
            self.currently_executing = Some((instruction, cycles_left - 1));
        }
        Ok(sprite_position)
    }

    fn execute(&mut self, instruction: Instruction) {
        use Instruction::*;
        match instruction {
            Noop => {}
            Addx(value) => {
                let register_value = self.sprite_position_mut();
                *register_value += value;
            }
        }
    }

    fn is_out_of_instructions(&self) -> bool {
        self.instructions.is_empty()
    }

    fn sprite_position(&self) -> i64 {
        *self.registers.get("X").unwrap()
    }

    fn sprite_position_mut(&mut self) -> &mut i64 {
        self.registers.get_mut("X").unwrap()
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.screen.len() {
            for col in 0..self.screen[0].len() {
                if self.is_lit(row, col) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Instruction {
    fn cycles_to_complete(&self) -> u64 {
        use Instruction::*;
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split_ascii_whitespace();
        let instruction = iter
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;
        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let value = iter
                    .next()
                    .ok_or_else(|| anyhow!("unexpected end of input"))
                    .and_then(|s| s.parse::<i64>().map_err(Error::from))?;

                Ok(Instruction::Addx(value))
            }
            _ => Err(anyhow!("unexpected instruction: {}", instruction)),
        }
    }
}

#[test]
fn part_1a() {
    let input = "noop
addx 3
addx -5";
    let mut computer = Computer::with_input(input).unwrap();
    assert_eq!(computer.sprite_position(), 1);
    computer.tick().unwrap();
    assert_eq!(computer.sprite_position(), 1);
    computer.tick().unwrap();
    assert_eq!(computer.sprite_position(), 1);
    computer.tick().unwrap();
    assert_eq!(computer.sprite_position(), 4);
    computer.tick().unwrap();
    assert_eq!(computer.sprite_position(), 4);
    computer.tick().unwrap();
    assert_eq!(computer.sprite_position(), -1);
}

#[test]
fn part_1b() {
    assert_eq!(
        sum_of_signal_strengths(test_input(), 20, 40).unwrap(),
        13140
    );
}

#[test]
fn part_2() {
    let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n";
    let mut computer = Computer::with_input(test_input()).unwrap();
    computer.run().unwrap();
    assert_eq!(format!("{}", computer), output);
}

#[cfg(test)]
fn test_input() -> &'static str {
    "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
}
