use anyhow::{anyhow, Error};
use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let input = include_str!("03.txt");
    println!("Part 1: {}", sum_of_priorities_of_shared_letters(input)?);
    println!("Part 2: {}", sum_of_priorities_of_badges(input)?);
    Ok(())
}

fn shared_letter(input: &str) -> Result<char, Error> {
    let (front, back) = input.split_at(input.len() / 2);
    let front: HashSet<char> = HashSet::from_iter(front.chars());
    let back: HashSet<char> = HashSet::from_iter(back.chars());
    let mut iter = front.intersection(&back);
    if let Some(shared_letter) = iter.next() {
        if iter.next().is_some() {
            Err(anyhow!(
                "more than one shared letter: {:?}",
                front.intersection(&back).collect::<Vec<_>>()
            ))
        } else {
            Ok(*shared_letter)
        }
    } else {
        Err(anyhow!("no shared letters"))
    }
}

fn priority(c: char) -> Result<i64, Error> {
    if c.is_ascii_uppercase() {
        Ok(c as i64 - 'A' as i64 + 27)
    } else if c.is_ascii_lowercase() {
        Ok(c as i64 - 'a' as i64 + 1)
    } else {
        Err(anyhow!("unexpected char: {}", c))
    }
}

fn sum_of_priorities_of_shared_letters(input: &str) -> Result<i64, Error> {
    let mut sum = 0;
    for line in input.lines() {
        let shared_letter = shared_letter(line)?;
        sum += priority(shared_letter)?;
    }
    Ok(sum)
}

fn badge(input: &str) -> Result<char, Error> {
    let mut shared_chars = HashSet::new();
    for line in input.lines() {
        if shared_chars.is_empty() {
            shared_chars.extend(line.chars());
        } else {
            let chars: HashSet<char> = HashSet::from_iter(line.chars());
            shared_chars = shared_chars.intersection(&chars).map(|&c| c).collect();
        }
    }
    if shared_chars.len() == 1 {
        Ok(shared_chars.into_iter().next().unwrap())
    } else {
        Err(anyhow!(
            "multiple (or no) shared characters found: {:?}",
            shared_chars
        ))
    }
}

fn sum_of_priorities_of_badges(input: &str) -> Result<i64, Error> {
    let input = input.trim();
    let mut start = 0;
    let mut sum = 0;
    for match_index in input.match_indices('\n').skip(2).step_by(3) {
        let badge = badge(&input[start..match_index.0])?;
        sum += priority(badge)?;
        start = match_index.0 + 1;
    }
    let badge = badge(&input[start..])?;
    sum += priority(badge)?;
    Ok(sum)
}

#[test]
fn part_1() {
    assert_eq!(shared_letter("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap(), 'p');
    assert_eq!(
        shared_letter("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap(),
        'L'
    );
    assert_eq!(shared_letter("PmmdzqPrVvPwwTWBwg").unwrap(), 'P');
    assert_eq!(
        shared_letter("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap(),
        'v'
    );
    assert_eq!(shared_letter("ttgJtRGJQctTZtZT").unwrap(), 't');
    assert_eq!(shared_letter("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap(), 's');

    assert_eq!(priority('a').unwrap(), 1);
    assert_eq!(priority('z').unwrap(), 26);
    assert_eq!(priority('A').unwrap(), 27);
    assert_eq!(priority('Z').unwrap(), 52);

    assert_eq!(
        sum_of_priorities_of_shared_letters(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .unwrap(),
        157
    )
}

#[test]
fn part_2() {
    assert_eq!(
        badge(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"
        )
        .unwrap(),
        'r'
    );
    assert_eq!(
        badge(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .unwrap(),
        'Z'
    );

    assert_eq!(
        sum_of_priorities_of_badges(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .unwrap(),
        70
    )
}
