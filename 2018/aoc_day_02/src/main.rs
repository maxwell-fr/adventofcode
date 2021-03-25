use std::collections::HashMap;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    println!("Checksum: {}", part_1(&problem_input)?);

    Ok(())
}

/// Executes logic for part 1 of Advent of Code 2018 day 2.
fn part_1(input: &String) -> AocResult<i32> {
    let mut three_count = 0;
    let mut two_count = 0;

    for ln in input.lines() {
        let mut letters_count: HashMap<char,u32> = HashMap::new();
        ln.chars().for_each(|c| *letters_count.entry(c).or_insert(0) += 1);
        if letters_count.iter().any(|l| *l.1 == 3) {
            three_count += 1;
        }
        if letters_count.iter().any(|l| *l.1 == 2) {
            two_count += 1;
        }
    }

    Ok(three_count * two_count)
}
