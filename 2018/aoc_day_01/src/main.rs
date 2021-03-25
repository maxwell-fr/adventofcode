use std::collections::HashSet;

use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input:String = get_problem_input()?; 

    println!("Resulting frequency: {}", part_1(&problem_input)?);
    println!("First repeated frequency: {}", part_2(&problem_input)?);

    Ok(())
}

fn part_1(input: &String) -> AocResult<i32>{
    let mut accumulator = 0;

    for ln in input.lines() {
        accumulator += ln.parse::<i32>()?;
    }
    
    Ok(accumulator)
}


fn part_2(input: &String) -> AocResult<i32>{
    let mut seen = HashSet::<i32>::new();

    let mut accumulator = 0;
    'outer: loop {
        for ln in input.lines() {
            accumulator += ln.parse::<i32>()?;
            if seen.contains(&accumulator) {
                break 'outer;
            }
            seen.insert(accumulator);
        }
    }
    
    Ok(accumulator)
}