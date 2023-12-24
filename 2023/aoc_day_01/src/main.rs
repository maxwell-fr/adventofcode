use aoc_util::aoc_util::*;


fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;


    part_1(&problem_input)?;
    part_2(&problem_input)?;

    Ok(())
}

fn part_1(problem_input: &String) -> AocResult<()> {
    let problem_lines = problem_input.lines();
    // obtain the first digit and last digit in each line to form a number
    // sum up all of these numbers to obtain result

    let mut accumulator: u32 = 0;
    for line in problem_lines {
        let (left, right) = find_digits_1(line)?;

        accumulator += (left * 10) + right;
    }

    println!("Pt 1 Result: {}", accumulator);
    Ok(())
}

fn part_2(problem_input: &String) -> AocResult<()>{
    let problem_lines = problem_input.lines();

    // obtain the first digit and last digit in each line to form a number
    // the digits might be words one, two, ..., nine
    // sum up all of these numbers to obtain result
    // sometimes the number words overlap :(

    let mut accumulator: u32 = 0;
    for line in problem_lines {
        let (left, right) = find_digits_2(line)?;

        accumulator += (left * 10) + right;
    }

    println!("Pt 2 Result: {}", accumulator);
    Ok(())
}

fn find_digits_1(s: &str) -> AocResult<(u32, u32)> {
    let mut left_digit: u32 = 0;
    let mut right_digit: u32 = 0;
    let mut left_pos: usize = 0;
    let mut right_pos: usize = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            left_digit = digit;
            left_pos = i;
            break
        }
    }

    for (i, c) in s.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            right_digit = digit;
            right_pos = i;
            break
        }
    }

    if s.chars().count() - right_pos <= left_pos {
        return Err("right <= left".into());
    }

    Ok((left_digit, right_digit))
}

fn find_digits_2(s: &str) -> AocResult<(u32, u32)> {
    let mut left_digit: u32 = 0;
    let mut right_digit: u32 = 0;
    let mut left_pos: usize = 0;
    let mut right_pos: usize = 0;

    let fwd = convert_wordnums_fwd(s.to_string());
    for (i, c) in fwd.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            left_digit = digit;
            left_pos = i;
            break
        }
    }

    let rev = convert_wordnums_rev(s.to_string());
    for (i, c) in rev.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            right_digit = digit;
            right_pos = i;
            break
        }
    }

    if rev.chars().count() - right_pos <= left_pos {
        return Err("right <= left".into());
    }

    Ok((left_digit, right_digit))
}

fn convert_wordnums_fwd(s: String) -> String {
    let wordnums = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let mut fixed = s.to_string();
    let mut leftmost: (usize, (&str, char)) = (usize::MAX, ("", ' '));
    for (word, num) in wordnums {
        if let Some(idx) = fixed.find(word) {
            if idx < leftmost.0 {
                leftmost = (idx, (word, num));
            }
        }
    }
    if leftmost.1.1 != ' ' {
        fixed.insert(leftmost.0, leftmost.1.1);
    }

    fixed
}

fn convert_wordnums_rev(s: String) -> String {
    let wordnums = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let mut fixed = s.to_string();
    let mut rightmost: (usize, (&str, char)) = (0, ("", ' '));
    for (word, num) in wordnums {
        if let Some(idx) = fixed.rfind(word) {
            if idx > rightmost.0 {
                rightmost = (idx, (word, num));
            }
        }
    }
    if rightmost.1.1 != ' ' {
        fixed.insert(rightmost.0, rightmost.1.1);
    }

    fixed
}