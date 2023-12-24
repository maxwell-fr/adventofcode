pub mod aoc_util {
    use std::env;
    use std::fs;

    pub type AocResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    pub fn get_problem_input() -> AocResult<String>{
        let args: Vec<String> = env::args().collect();

        match args.len() {
            2 => {
                println!("Will use file: {}", args[1]);
                Ok(fs::read_to_string(&args[1])?)
            },
            _ => {
                Err("Need one filename to open.".into())
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
