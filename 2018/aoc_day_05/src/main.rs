use std::time::Instant;
use std::collections::HashMap;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    let start = Instant::now();
    println!("1: units remaining: {}", part_1(&problem_input)?);
    let after_1 = Instant::now();
//    println!("2:  {:?}", part_2(&problem_input)?);
    let after_2 = Instant::now();
    println!("1: {:?}  2: {:?}", after_1 - start, after_2 - after_1 );

    Ok(())
}

fn part_1(input: &str) -> AocResult<u32> {
    assert!(input.is_ascii());
    let input = input.as_bytes();
    let input_len = input.len();

    let mut left = 0;
    let mut right = 1;
    loop {
        if annihilate(&input[left], &input[right]) {
            
        }
    }
    Ok(0)
}

///checks if two bytes, when interpreted as ascii characters, should annihilate
fn annihilate(a: &u8, b: &u8) -> bool {
    true
}
