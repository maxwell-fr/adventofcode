use std::collections::HashMap;
use aoc_util::aoc_util::*;

//
enum Parts {
    One, Two
}

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;

    let part1 = solve(&problem_input, Parts::One)?;
    println!("Part 1: {part1}");

    let part2 = solve(&problem_input, Parts::Two)?;
    println!("Part 2: {part2}");

    Ok(())
}

/// Solve the problem.
///
/// Obtain the first digit and last digit in each line to form a number.
/// Sum up all of these numbers to obtain result.
/// For Part 2, do the same thing, but treat certain words as numbers when building
/// Pass the appropriate Parts value to select part.
fn solve(problem_input: &String, part: Parts) -> AocResult<u32> {
    let problem_lines = problem_input.lines();

    let mut accumulator: u32 = 0;
    for line in problem_lines {
        if let Some((left, right)) = find_digits(&line.to_string(), matches!(part, Parts::Two)) {
            accumulator += (left * 10) + right;
        }
    }

    Ok(accumulator)
}

/// Position in input string
type Position = usize;
/// Value of digit
type Value = u32;
/// HashMap of values keyed by position
type PVHashMap = HashMap<Position, Value>;
/// Tuple representing a pair of digits (Left, Right)
type DigitPair = (u32, u32);

/// Search for the first and last digits in a given string, using the words table if specified.
fn find_digits(s: &String, use_words: bool) -> Option<DigitPair>{
    let lookup_words: Vec<(&str, Value)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let lookup_nums: Vec<(&str, Value)> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];


    let mut positions = PVHashMap::new();
    find_lookup_words(s, &lookup_nums, &mut positions);

    if use_words {
        find_lookup_words(s, &lookup_words, &mut positions);
    }

    let mut keys: Vec<&usize> = positions.keys().collect();
    keys.sort_by_key(|&&k| k);

    //if there is only one digit, it counts for the left and the right values
    if keys.len() >= 1 {
        Some((positions[keys[0]], positions[keys[keys.len() - 1]]))
    }
    else {
        None
    }
}

/// Check for each entry in the lookup table and add it and its value to the positions hashmap.
fn find_lookup_words(s: &String, lookup: &Vec<(&str, u32)>, positions: &mut PVHashMap){
    for word in lookup {
        for (idx, _) in s.match_indices(word.0) {
            positions.insert(idx, word.1) ;
        }
    }
}

/// Tests to verify changes. Answer spoilers are here!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_p1_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, Parts::One).unwrap(), 54634);
    }
    #[test]
    fn check_p2_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, Parts::Two).unwrap(), 53855);
    }
}

