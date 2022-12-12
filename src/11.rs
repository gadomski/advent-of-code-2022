use anyhow::{anyhow, Error, Result};
use std::{
    collections::HashMap,
    str::{FromStr, Lines},
};

fn main() -> Result<()> {
    let input = include_str!("11.txt");
    let mut monkey_business = MonkeyBusiness::from_input(input)?;
    monkey_business.execute_many(20);
    println!("Part 1: {}", monkey_business.level()?);
    let mut monkey_business = MonkeyBusiness::from_input(input)?;
    monkey_business.divide_by_three = false;
    monkey_business.execute_many(10_000);
    println!("Part 2: {}", monkey_business.level()?);
    Ok(())
}

#[derive(Debug)]
struct MonkeyBusiness {
    monkeys: HashMap<u8, Monkey>,
    numbers: Vec<u8>,
    divide_by_three: bool,
    least_common_multiple: i64,
}

#[derive(Debug)]
struct Monkey {
    number: u8,
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    inspections: u64,
}

#[derive(Debug)]
struct Operation {
    a: Operand,
    b: Operand,
    function: Function,
}

#[derive(Debug)]
enum Operand {
    Old,
    Const(i64),
}

#[derive(Debug)]
enum Function {
    Add,
    Mul,
}

#[derive(Debug)]
struct Test {
    divisible_by: i64,
    if_true: u8,
    if_false: u8,
}

impl MonkeyBusiness {
    fn from_input(input: &str) -> Result<MonkeyBusiness> {
        let mut monkey_business = MonkeyBusiness::new();
        for lines in input.split("\n\n") {
            monkey_business.add_monkey(lines.parse()?)?;
        }
        Ok(monkey_business)
    }

    fn new() -> MonkeyBusiness {
        MonkeyBusiness {
            monkeys: HashMap::new(),
            numbers: Vec::new(),
            divide_by_three: true,
            least_common_multiple: 0,
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) -> Result<()> {
        let number = monkey.number;
        if let Some(monkey) = self.monkeys.insert(number, monkey) {
            Err(anyhow!("duplicate monkey: {:?}", monkey))
        } else {
            self.numbers.push(number);
            self.numbers.sort();
            self.least_common_multiple = self
                .monkeys
                .values()
                .map(|monkey| monkey.test.divisible_by)
                .product();
            Ok(())
        }
    }

    fn execute_many(&mut self, count: usize) {
        for _ in 0..count {
            self.execute();
        }
    }

    fn execute(&mut self) {
        for number in &self.numbers {
            for (item, target) in self
                .monkeys
                .get_mut(number)
                .unwrap()
                .inspect_and_throw(self.divide_by_three)
            {
                let item = item % self.least_common_multiple;
                self.monkeys.get_mut(&target).unwrap().items.push(item);
            }
        }
    }

    fn level(&self) -> Result<u64> {
        let mut monkeys: Vec<_> = self.monkeys.values().collect();
        if monkeys.len() < 2 {
            Err(anyhow!("too few monkeys: {}", monkeys.len()))
        } else {
            monkeys.sort_by_key(|monkey| monkey.inspections);
            Ok(monkeys[monkeys.len() - 2].inspections * monkeys[monkeys.len() - 1].inspections)
        }
    }
}

impl Monkey {
    fn inspect_and_throw(&mut self, divide_by_three: bool) -> Vec<(i64, u8)> {
        let mut throws = Vec::new();
        let items = std::mem::take(&mut self.items);
        for item in items {
            let mut item = self.inspect(item);
            if divide_by_three {
                item = item / 3;
            }
            let target = self.test(item);
            throws.push((item, target));
            self.inspections += 1;
        }
        throws
    }

    fn inspect(&self, item: i64) -> i64 {
        self.operation.function.call(
            self.operation.a.resolve(item),
            self.operation.b.resolve(item),
        )
    }

    fn test(&self, item: i64) -> u8 {
        if (item % self.test.divisible_by) == 0 {
            self.test.if_true
        } else {
            self.test.if_false
        }
    }
}

impl FromStr for Monkey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ref mut lines = s.lines();
        let line = next_line(lines)?;

        // Monkey
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "Monkey")?;
        let number_str = next_str(words)?;
        let number: u8 = if number_str.ends_with(':') {
            number_str[0..(number_str.len() - 1)].parse()?
        } else {
            return Err(anyhow!("invalid number str: {}", number_str));
        };

        // Starting items
        let line = next_line(lines)?.trim();
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "Starting")?;
        expect_next(&mut words, "items:")?;
        let mut items = Vec::new();
        for mut word in words {
            if word.ends_with(',') {
                word = &word[0..(word.len() - 1)];
            }
            let item: i64 = word.parse()?;
            items.push(item);
        }

        // Opeartion
        let line = next_line(lines)?.trim();
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "Operation:")?;
        expect_next(&mut words, "new")?;
        expect_next(&mut words, "=")?;
        let a: Operand = next(&mut words)?;
        let function: Function = next(&mut words)?;
        let b: Operand = next(words)?;
        let operation = Operation { a, function, b };

        // Test
        let line = next_line(lines)?.trim();
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "Test:")?;
        expect_next(&mut words, "divisible")?;
        expect_next(&mut words, "by")?;
        let divisible_by: i64 = words
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))
            .and_then(|s| s.parse().map_err(Error::from))?;

        // If true
        let line = next_line(lines)?.trim();
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "If")?;
        expect_next(&mut words, "true:")?;
        expect_next(&mut words, "throw")?;
        expect_next(&mut words, "to")?;
        expect_next(&mut words, "monkey")?;
        let if_true: u8 = words
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))
            .and_then(|s| s.parse().map_err(Error::from))?;

        // If false
        let line = next_line(lines)?.trim();
        let mut words = line.split_ascii_whitespace();
        expect_next(&mut words, "If")?;
        expect_next(&mut words, "false:")?;
        expect_next(&mut words, "throw")?;
        expect_next(&mut words, "to")?;
        expect_next(&mut words, "monkey")?;
        let if_false: u8 = words
            .next()
            .ok_or_else(|| anyhow!("unexpected end of input"))
            .and_then(|s| s.parse().map_err(Error::from))?;

        let test = Test {
            divisible_by,
            if_true,
            if_false,
        };

        Ok(Monkey {
            number,
            items,
            operation,
            test,
            inspections: 0,
        })
    }
}

impl Operand {
    fn resolve(&self, old: i64) -> i64 {
        use Operand::*;
        match self {
            Old => old,
            Const(value) => *value,
        }
    }
}

impl FromStr for Operand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => s
                .parse::<i64>()
                .map(|v| Operand::Const(v))
                .map_err(Error::from),
        }
    }
}

impl Function {
    fn call(&self, a: i64, b: i64) -> i64 {
        use Function::*;
        match self {
            Add => a + b,
            Mul => a * b,
        }
    }
}

impl FromStr for Function {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Function::Mul),
            "+" => Ok(Function::Add),
            _ => Err(anyhow!("unexpected function: {}", s)),
        }
    }
}

fn next<'a, I, T>(iter: I) -> Result<T>
where
    I: Iterator<Item = &'a str>,
    T: FromStr<Err = Error>,
{
    let s = next_str(iter)?;
    s.parse()
}

fn next_str<'a, I>(mut iter: I) -> Result<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    iter.next()
        .ok_or_else(|| anyhow!("unexpected end of iterator"))
}

fn expect_next<'a, I>(iter: I, expected: &str) -> Result<()>
where
    I: Iterator<Item = &'a str>,
{
    let actual: &str = next_str(iter)?;
    if actual != expected {
        Err(anyhow!("actual={}, expected={}", actual, expected))
    } else {
        Ok(())
    }
}

fn next_line<'a>(iter: &'_ mut Lines<'a>) -> Result<&'a str> {
    iter.next().ok_or_else(|| anyhow!("expected another line"))
}

#[test]
fn part_1() {
    let mut monkey_business = MonkeyBusiness::from_input(test_input()).unwrap();
    monkey_business.execute_many(20);
    assert_eq!(monkey_business.level().unwrap(), 10605);
}

#[test]
fn part_2() {
    let mut monkey_business = MonkeyBusiness::from_input(test_input()).unwrap();
    monkey_business.divide_by_three = false;
    monkey_business.execute_many(10_000);
    assert_eq!(monkey_business.level().unwrap(), 2713310158);
}

#[cfg(test)]
fn test_input() -> &'static str {
    "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
}
