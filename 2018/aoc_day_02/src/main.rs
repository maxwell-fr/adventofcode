use std::collections::HashMap;
use aoc_util::aoc_util::*;
use std::time::Instant;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    println!("Checksum: {}", part_1(&problem_input)?);
    let start = Instant::now();
    println!("A: ID differing by one character, common chars only: {}", part_2a(&problem_input)?);
    let after_a = Instant::now();
    println!("B: ID differing by one character, common chars only: {}", part_2b(&problem_input)?);
    let after_b = Instant::now();

    println!("2a: {:?}  2b: {:?}", after_a - start, after_b - after_a);
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

fn part_2a(input: &String) -> AocResult<String> {
    for ln_a in input.lines() {
        for ln_b in input.lines().skip(1) {
            if ln_a.len() != ln_b.len() {
                continue;
            }

            let mut common_c = String::with_capacity(ln_a.len());
            ln_a.chars().zip(ln_b.chars()).filter(|(a,b)| a==b).for_each(|(a,_b)| common_c.push(a));
            if common_c.len() == ln_a.len() - 1 {
                return Ok(common_c);
            }
        }
    }

    Err("Nothing found.".into())
}

fn part_2b(input: &String) -> AocResult<String> {
    for ln_a in input.lines() {
        for ln_b in input.lines().skip(1) {
            if ln_a.len() != ln_b.len() {
                continue;
            }

            let mut common_c = String::with_capacity(ln_a.len());
            let mut diff_count = 0;
            for (a, b) in ln_a.chars().zip(ln_b.chars()){
                if a != b {
                    diff_count += 1;
                    if diff_count > 1 {
                        break;
                    }
                }
                else {
                    common_c.push(a);
                }
            }

            if diff_count == 1 {
                return Ok(common_c);
            }
        }
    }

    Err("Nothing found.".into())
}