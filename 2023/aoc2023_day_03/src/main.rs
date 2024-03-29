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

/// number of columns in the puzzle data
const GRID_COLS: usize = 140;
/// number of rows in the puzzle data
const GRID_ROWS: usize = 140;

/// Grid Type
struct Grid {
    grid: Vec<Vec<u8>>,
}
enum CellType {
    Nothing,
    Digit(u8),
    Symbol(u8),
}

impl Grid {
    fn check_cell(&self, row: i32, col: i32, row_delta: i32, col_delta: i32) -> CellType {
        let check_row = row + row_delta;
        let check_col = col + col_delta;

        if check_row >= 0 && check_row < GRID_ROWS as i32
            && check_col >= 0 && check_col < GRID_COLS as i32 {
            let check_row = check_row as usize;
            let check_col = check_col as usize;
            if self.grid[check_row][check_col].is_ascii_digit() {
                CellType::Digit(self.grid[check_row][check_col] - 48) //48 is ASCII '0'
            }
            else if self.grid[check_row][check_col] != '.' as u8 {
                CellType::Symbol(self.grid[check_row][check_col])
            }
            else {
                CellType::Nothing
            }
        }
        else {
            CellType::Nothing
        }
    }

    fn touching_symbol(&self, row: i32, col: i32, _sym: u8) -> bool {
        matches!(self.check_cell(row, col, -1, -1), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, -1, 0), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, -1, 1), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, 0, -1), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, 0, 1), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, 1, -1), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, 1, 0), CellType::Symbol(..)) ||
        matches!(self.check_cell(row, col, 1, 1), CellType::Symbol(..))
    }

    fn get_number(&self, row: i32, col: i32) -> Option<i32> {
        match self.check_cell(row, col, 0, 0) {
            CellType::Nothing => {None}
            CellType::Digit(_) => {
                let mut start_offset: i32 = -1;
                //find leftmost digit
                while let CellType::Digit(_) = self.check_cell(row, col, 0, start_offset) {
                    start_offset -= 1;
                }
                //find rightmost digit and build value
                let mut value: i32 = 0;
                let mut end_offset: i32 = start_offset + 1;
                while let CellType::Digit(x) = self.check_cell(row, col, 0, end_offset) {
                    value = (value * 10) + x as i32;
                    end_offset += 1;
                }
                Some(value)
            }
            CellType::Symbol(_) => {None}
        }
    }
}

impl Index<usize> for Grid {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl From<Vec<Vec<u8>>> for Grid {
    fn from(value: Vec<Vec<u8>>) -> Self {
        assert_eq!(value.len(), GRID_ROWS);
        value.iter().for_each(|v| assert_eq!(v.len(), GRID_COLS));

        Grid {
            grid: value
        }
    }
}

/// Solve the problem.
///
/// invokes the solvers depending on the part parameter
fn solve(problem_input: &String, part: AocParts) -> AocResult<i32> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in problem_input.lines() {
        grid.push(line.as_bytes().to_vec())
    }

    match part {
        AocParts::One => part_1(&grid.into()),
        AocParts::Two => part_2(&grid.into())
    }

}

/// for part 1, find all of the numbers that "touch" a non-. symbol and sum them up
fn part_1(grid: &Grid) -> AocResult<i32>{
    let mut values: Vec<i32> = Vec::new();

    for row in 0..GRID_ROWS as i32 {
        let mut col: i32 = 0;
        loop {
            match grid.check_cell(row, col, 0, 0) {
                CellType::Nothing => {
                    col += 1;
                }
                CellType::Digit(x) => {
                    let mut value = x as i32;
                    let mut touching = grid.touching_symbol(row, col, 0);
                    col += 1;
                    loop { // seek the end of the digits
                        match grid.check_cell(row, col, 0, 0) {
                            CellType::Nothing => {
                                break;
                            }
                            CellType::Digit(x) => {
                                value = (value * 10) + x as i32;
                                touching = touching || grid.touching_symbol(row, col, 0);
                                col += 1;
                            }
                            CellType::Symbol(_) => {
                                break;
                            }
                        }
                        if col >= GRID_COLS as i32 {
                            break;
                        }
                    }
                    if touching {
                        values.push(value);
                    }
                }
                CellType::Symbol(_) => {
                    col += 1;
                }
            }
            if col >= GRID_COLS as i32 {
                break;
            }
        }
    }

    Ok(values.iter().sum())
}

/// for part 2, find all of the "gears" (*) that touch exactly two numbers
/// and sum the products of each pair
fn part_2(grid: &Grid) -> AocResult<i32> {
    let mut values: Vec<i32> = Vec::new();

    for row in 0..GRID_ROWS as i32 {
        let mut col: i32 = 0;
        loop {
            match grid.check_cell(row, col, 0, 0) {
                CellType::Nothing => {
                    col += 1;
                }
                CellType::Digit(_) => {
                    col += 1;
                }
                CellType::Symbol(c) if c == '*' as u8 => {
                    let mut numbers: Vec<i32> = Vec::new();
                    //above symbol
                    if let Some(n) = grid.get_number(row - 1, col ) { // above
                        numbers.push(n);
                    }
                    else { // corners - there must be a gap between
                        if let Some(n) = grid.get_number(row - 1, col - 1) { //upper left
                            numbers.push(n);
                        }
                        if let Some(n) = grid.get_number(row - 1, col + 1) { //upper right
                            numbers.push(n);
                        }
                    }

                    //same row as symbol
                    if let Some(n) = grid.get_number(row, col - 1) { //left
                        numbers.push(n);
                    }
                    if let Some(n) = grid.get_number(row, col + 1) { //right
                        numbers.push(n);
                    }

                    //below symbol
                    if let Some(n) = grid.get_number(row + 1, col ) { //below
                        numbers.push(n);
                    }
                    else { //corners - there must be a gap between
                        if let Some(n) = grid.get_number(row + 1, col - 1) { //bottom left
                            numbers.push(n);
                        }
                        if let Some(n) = grid.get_number(row + 1, col + 1) { //bottom right
                            numbers.push(n);
                        }
                    }

                    // only multiply and store if there are exactly two numbers
                    if numbers.len() == 2 {
                        values.push(numbers.iter().product());
                    }

                    col += 1;
                }
                CellType::Symbol(_) => {
                    col += 1;
                }
            }
            if col >= GRID_COLS as i32 {
                break;
            }
        }
    }

    Ok(values.iter().sum())
}

/// Tests to verify changes. Answer spoilers are here!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_p1_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, AocParts::One).unwrap(), 532445);
    }
    #[test]
    fn check_p2_results() {
        let problem_input = get_problem_input().unwrap();
        assert_eq!(solve(&problem_input, AocParts::Two).unwrap(), 79842967);
    }
}
