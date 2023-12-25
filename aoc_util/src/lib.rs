pub mod aoc_util {
    use std::env;
    use std::fs;

    /// Convenience type for a catch-all-errors Result
    pub type AocResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// Indicate which part of the problem we're on
    pub enum AocParts {
        One, Two
    }
    /// Fetch the problem input as one big String from a file.
    /// The file is either the one specified on the command line or input.txt in the cwd.
    pub fn get_problem_input() -> AocResult<String>{
        let args: Vec<String> = env::args().collect();

        let file_to_read = match args.len() {
            2 => {
                &args[1]
            },
            _ => {
                "input.txt"
            }
        };

        println!("Using file: {}", file_to_read);
        Ok(fs::read_to_string(file_to_read)?)
    }
}


