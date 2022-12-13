use anyhow::{anyhow, Error, Result};
use std::{
    cmp::Ordering,
    iter::Peekable,
    str::{Chars, FromStr},
};

fn main() -> Result<()> {
    let input = include_str!("13.txt");
    println!("Part 1: {}", sum_of_indices_in_correct_order(input)?);
    println!("Part 2: {}", decoder_key(input)?);
    Ok(())
}

fn decoder_key(input: &str) -> Result<usize> {
    let first: Packet = "[[2]]".parse()?;
    let second: Packet = "[[6]]".parse()?;
    let mut packets: Vec<Packet> = vec![first.clone(), second.clone()];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        packets.push(line.parse()?);
    }
    packets.sort();
    let mut first_position = None;
    let mut second_position = None;
    for (i, packet) in packets.into_iter().enumerate() {
        if packet == first {
            first_position = Some(i + 1);
        } else if packet == second {
            second_position = Some(i + 1);
        }
    }
    Ok(first_position.unwrap() * second_position.unwrap())
}

fn sum_of_indices_in_correct_order(input: &str) -> Result<usize> {
    let mut sum = 0;
    for (i, lines) in input.split("\n\n").enumerate() {
        let mut iter = lines.lines();
        let left: Packet = iter
            .next()
            .ok_or_else(|| anyhow!("missing first packet"))?
            .parse()?;
        let right: Packet = iter
            .next()
            .ok_or_else(|| anyhow!("missing second packet"))?
            .parse()?;
        if left < right {
            sum += i + 1;
        }
    }
    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet(Vec<Value>);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Integer(i64),
    List(Vec<Value>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        cmp_list(&self.0, &other.0)
    }
}

impl FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Packet> {
        if !s.starts_with('[') {
            Err(anyhow!("packet must start with a '["))
        } else if !s.ends_with(']') {
            Err(anyhow!("packet must end with a ']'"))
        } else {
            let list = parse_list(&mut s[1..(s.len() - 1)].chars().peekable())?;
            Ok(Packet(list))
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        use Value::*;
        match self {
            List(left) => match other {
                List(right) => cmp_list(left, right),
                Integer(right) => cmp_list(left, &[Integer(*right)]),
            },
            Integer(left) => match other {
                List(right) => cmp_list(&[Integer(*left)], right),
                Integer(right) => left.cmp(right),
            },
        }
    }
}

fn parse_list(chars: &mut Peekable<Chars<'_>>) -> Result<Vec<Value>> {
    use Value::*;
    let mut values = Vec::new();
    while let Some(peeked) = chars.peek() {
        if *peeked == ']' {
            break;
        } else if *peeked == '[' {
            let _ = chars.next().unwrap();
            let list = parse_list(chars)?;
            let c = chars
                .next()
                .ok_or_else(|| anyhow!("unexpected end of input in parse_list"))?;
            if c != ']' {
                return Err(anyhow!("unclosed list"));
            }
            values.push(List(list));
        } else if *peeked == ',' {
            let _ = chars.next().unwrap();
        } else {
            let integer = parse_integer(chars)?;
            values.push(Integer(integer));
        }
    }
    Ok(values)
}

fn parse_integer(chars: &mut Peekable<Chars<'_>>) -> Result<i64> {
    let mut value = String::new();
    while let Some(peeked) = chars.peek() {
        if *peeked == ']' || *peeked == ',' {
            break;
        } else {
            let c = chars.next().unwrap();
            value.push(c);
        }
    }
    value.parse().map_err(Error::from)
}

fn cmp_list(left: &[Value], right: &[Value]) -> Ordering {
    use Ordering::*;
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    while let Some(left) = left_iter.next() {
        if let Some(right) = right_iter.next() {
            if let Some(ordering) = left.partial_cmp(right) {
                if !matches!(ordering, Equal) {
                    return ordering;
                }
            }
        } else {
            return Ordering::Greater;
        }
    }
    if right_iter.next().is_some() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

#[test]
fn part_1_1() {
    let left: Packet = "[1,1,3,1,1]".parse().unwrap();
    let right: Packet = "[1,1,5,1,1]".parse().unwrap();
    assert!(left < right);
}

#[test]
fn part_1_2() {
    let left: Packet = "[[1],[2,3,4]]".parse().unwrap();
    let right: Packet = "[[1],4]".parse().unwrap();
    assert!(left < right);
}

#[test]
fn part_1_3() {
    let left: Packet = "[9]".parse().unwrap();
    let right: Packet = "[[8,7,6]]".parse().unwrap();
    assert!(left > right);
}

#[test]
fn part_1_4() {
    let left: Packet = "[[4,4],4,4]".parse().unwrap();
    let right: Packet = "[[4,4],4,4,4]".parse().unwrap();
    assert!(left < right);
}

#[test]
fn part_1_5() {
    let left: Packet = "[7,7,7,7]".parse().unwrap();
    let right: Packet = "[7,7,7]".parse().unwrap();
    assert!(left > right);
}

#[test]
fn part_1_6() {
    let left: Packet = "[]".parse().unwrap();
    let right: Packet = "[3]".parse().unwrap();
    assert!(left < right);
}

#[test]
fn part_1_7() {
    let left: Packet = "[[[]]]".parse().unwrap();
    let right: Packet = "[[]]".parse().unwrap();
    assert!(left > right);
}

#[test]
fn part_1_8() {
    let left: Packet = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
    let right: Packet = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();
    assert!(left > right);
}

#[test]
fn part_1() {
    assert_eq!(sum_of_indices_in_correct_order(test_input()).unwrap(), 13);
}

#[test]
fn part_2() {
    assert_eq!(decoder_key(test_input()).unwrap(), 140);
}

#[cfg(test)]
fn test_input() -> &'static str {
    "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
}
