use anyhow::{anyhow, Error};
use std::collections::{HashMap, VecDeque};

fn main() -> Result<(), Error> {
    let input = include_str!("06.txt");
    println!(
        "Part 1: {}",
        start_of_packet(input).ok_or_else(|| anyhow!("no start of packet found: {}", input))?
    );
    println!(
        "Part 2: {}",
        start_of_message(input).ok_or_else(|| anyhow!("no start of message found: {}", input))?
    );
    Ok(())
}

fn start_of_packet(input: &str) -> Option<usize> {
    first_unique_set(input, 4)
}

fn start_of_message(input: &str) -> Option<usize> {
    first_unique_set(input, 14)
}

fn first_unique_set(input: &str, len: usize) -> Option<usize> {
    let mut buffer = VecDeque::new();
    let mut chars = HashMap::new();
    for (i, c) in input.chars().enumerate() {
        buffer.push_back(c);
        let entry = chars.entry(c).or_insert(0);
        *entry += 1;
        if buffer.len() > len {
            let c = buffer
                .pop_front()
                .expect("there must be characters in the buffer");
            let count = chars
                .get_mut(&c)
                .expect("the character should be in the set");
            *count -= 1;
            if *count == 0 {
                chars.remove(&c);
            }
        }
        if buffer.len() == len {
            if chars.values().all(|count| *count == 1) {
                return Some(i + 1);
            }
        }
    }
    None
}

#[test]
fn part_1() {
    assert_eq!(
        start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
        7
    );
    assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
    assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
    assert_eq!(
        start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
        10
    );
    assert_eq!(
        start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
        11
    );
}

#[test]
fn part_2() {
    assert_eq!(
        start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(),
        19
    );
    assert_eq!(
        start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(),
        23
    );
    assert_eq!(
        start_of_message("nppdvjthqldpwncqszvftbrmjlhg").unwrap(),
        23
    );
    assert_eq!(
        start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(),
        29
    );
    assert_eq!(
        start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(),
        26
    );
}
