use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;

    let part1 = solve(&problem_input, AocParts::One)?;
    println!("Part 1: {part1}");
    let part2 = solve(&problem_input, AocParts::Two)?;
    println!("Part 2: {part2}");
    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    red_max: u32,
    green_max: u32,
    blue_max: u32,
}

/// Solve the problem.
///
/// For part 1, find the sum of the IDs of the games that meet the eligibility requirements
/// (no more than 12, 13, and 14 of red, green, and blue cubes respectively
///
/// For part 2, find the sum of the products of the minimum cubes needed for every game.
fn solve(problem_input: &String, part: AocParts) -> AocResult<u32> {
    let mut games: Vec<Game> = Vec::new();

    // we could do fancy stuff like regexes or somesuch but just repeatedly splitting is sufficient
    for line in problem_input.lines() {
        // split on game id vs rounds list
        let split1: Vec<&str> = line.split(":").collect();

        let mut game = Game {
            id: split1[0].to_string().replace("Game ", "").parse::<u32>()?,
            red_max: 0,
            green_max: 0,
            blue_max: 0,
        };

        // split the rounds list and iterate
        for round in split1[1].split(";") {
            //split the list of cube counts and iterate
            for cubes in round.split(",") {
                // split the round into cubes: count color
                let cube_split: Vec<&str> = cubes.trim().split(" ").collect();
                assert_eq!(cube_split.len(), 2);
                let value = cube_split[0].parse::<u32>()?;
                match cube_split[1] {
                    "red" => {
                        if value > game.red_max {
                            game.red_max = value;
                        }
                    }
                    "green" => {
                        if value > game.green_max {
                            game.green_max = value;
                        }
                    }
                    "blue" => {
                        if value > game.blue_max {
                            game.blue_max = value;
                        }
                    }
                    _ => {
                        panic!("Something weird in the input on game {}", game.id);
                    }
                }
            }
        }

        games.push(game);
    }

    match part {
        AocParts::One => {
            const RED_CUBES: u32 = 12;
            const GREEN_CUBES: u32 = 13;
            const BLUE_CUBES: u32 = 14;
            let eligible_games_id_sum = games
                .iter()
                .filter(|g| {
                    g.red_max <= RED_CUBES && g.green_max <= GREEN_CUBES && g.blue_max <= BLUE_CUBES
                })
                .map(|g| g.id)
                .sum();
            Ok(eligible_games_id_sum)
        }
        AocParts::Two => {
            let game_cube_power_sum = games
                .iter()
                .map(|g| g.red_max * g.blue_max * g.green_max)
                .sum();
            Ok(game_cube_power_sum)
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
        assert_eq!(solve(&problem_input, AocParts::One).unwrap(), 2006);
    }
    #[test]
    fn check_p2_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, AocParts::Two).unwrap(), 84911);
    }
}
