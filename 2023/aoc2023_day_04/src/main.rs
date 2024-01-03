use std::ops::Index;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;

    let part1 = solve(&problem_input, AocParts::One)?;
    println!("Part 1: {part1}");
    let part2 = solve(&problem_input, AocParts::Two)?;
    println!("Part 2: {part2}");
    Ok(())
}


/// Solve the problem.
///
/// invokes the solvers depending on the part parameter
fn solve(problem_input: &String, part: AocParts) -> AocResult<i32> {
    todo!();
}


/// Tests to verify changes. Answer spoilers are here!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_p1_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, AocParts::One).unwrap(), 0);
    }
    #[test]
    fn check_p2_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, AocParts::Two).unwrap(), 0);
    }
}
