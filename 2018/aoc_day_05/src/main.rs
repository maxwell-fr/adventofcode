use std::time::Instant;
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

fn part_1(in_str: &str) -> AocResult<u32> {
    assert!(in_str.is_ascii());
    let mut input  = Vec::from(in_str.as_bytes());

    let mut left = 0;
    loop {
        if annihilate(&input[left], &input[left+1]) {
            input.remove(left+1);
            input.remove(left);
            if left > 0 {
                left -= 1;
            }
            if left == input.len() - 1 {
                break;
            }
        }
        else {
            left += 1;
            if left >= input.len() - 1  {
                break;
            }
        }
    }
    Ok(input.len() as u32 - 1) //minus one to drop the terminating nul
}

///checks if two bytes, when interpreted as ascii characters, should annihilate
fn annihilate(a: &u8, b: &u8) -> bool {
    return a > b && a - b == 32 || b > a && b - a == 32
}
