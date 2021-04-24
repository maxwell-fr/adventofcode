use std::time::Instant;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    let start = Instant::now();
    println!("1: units remaining: {}", part_1(&problem_input)?);
    let after_1 = Instant::now();
    println!("2: units remaining (best) {:?}", part_2(&problem_input)?);
    let after_2 = Instant::now();
    println!("1: {:?}  2: {:?}", after_1 - start, after_2 - after_1 );

    Ok(())
}

fn part_1(in_str: &str) -> AocResult<u32> {
    assert!(in_str.is_ascii());
    let mut input: Vec<u8> = Vec::from(in_str.as_bytes());
    let mut ctrl_chars = 0;
    let mut left = 0;
    loop {
        if input[left] < 32 {
            ctrl_chars += 1;
        }
        if annihilate(&input[left], &input[left+1]) {
            input.remove(left+1);
            input.remove(left);
            if left > 0 {
                left -= 1;
            }
        }
        else {
            left += 1;
        }
        if left >= input.len() - 1 {
            break;
        }
    }
    Ok(input.len() as u32 - ctrl_chars)
}

fn part_2(in_str: &str) -> AocResult<u32> {
    assert!(in_str.is_ascii());
    let mut shortest =  in_str.len() as u32;

    for (a,b) in ('A'..='Z').zip('a'..='z') {
        let input: String = in_str.to_owned().chars().filter(|&x| x != a && x != b).collect();
        let s = part_1(&input)?;
        if s <= shortest {
            shortest = s;
        }
    }
    Ok(shortest)
}

///checks if two bytes, when interpreted as ascii characters, should annihilate
fn annihilate(a: &u8, b: &u8) -> bool {
    a > b && a - b == 32 || b > a && b - a == 32
}
