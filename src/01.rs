use anyhow::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("01.txt");
    println!(
        "Part 1: {}",
        calories_carried_by_elf_with_most_calories(input)?,
    );
    println!(
        "Part 1: {}",
        calories_carried_by_elves_with_most_calories(input, 3)?,
    );
    Ok(())
}

fn calories_carried_by_elf_with_most_calories(input: &str) -> Result<i64, Error> {
    calories_carried_by_elves_with_most_calories(input, 1)
}

fn calories_carried_by_elves_with_most_calories(input: &str, n: usize) -> Result<i64, Error> {
    let mut elves = Vec::new();
    for elf in input.split("\n\n") {
        let mut calories = 0;
        for line in elf.lines() {
            calories += line.parse::<i64>()?;
        }
        elves.push(calories)
    }
    elves.sort();
    Ok(elves.into_iter().rev().take(n).sum())
}

#[test]
fn example_1() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    assert_eq!(
        calories_carried_by_elf_with_most_calories(input).unwrap(),
        24000
    );
}

#[test]
fn example_2() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    assert_eq!(
        calories_carried_by_elves_with_most_calories(input, 3).unwrap(),
        45000
    );
}
