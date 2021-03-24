use std::env;
use std::fs;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => println!("Will use file: {}", args[1]),
        _ => {
            println!("Need one filename to open.");
            return Ok(());
        }
    };

    let problem_input:String = fs::read_to_string(&args[1])?;

    println!("Resulting frequency: {}", part_1(&problem_input)?);
    println!("First repeated frequency: {}", part_2(&problem_input)?);

    Ok(())
}

fn part_1(input: &String) -> Result<i32>{
    let mut accumulator = 0;

    for ln in input.lines() {
        accumulator += ln.parse::<i32>()?;
    }
    
    Ok(accumulator)
}


fn part_2(input: &String) -> Result<i32>{
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