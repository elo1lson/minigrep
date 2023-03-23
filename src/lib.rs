/// ## Author
/// - Name: elo1lson
/// - E-mail: eloilsonfontenele2@gmail.com
/// 2023-03-22

pub mod cli {
    use std::error::Error;
    use std::{env, fs};

    pub struct Config {
        pub query: String,
        pub file_path: String,
        ignore_case: bool,
    }

    impl Config {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query: String = args[1].clone();
            let file_path: String = args[2].clone();
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&str> = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        if results.len() == 0 {
            results.push("No matches in the file.")
        }
        results
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query: String = query.to_lowercase();
        let mut results: Vec<&str> = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents: String = fs::read_to_string(config.file_path)?;

        println!("{}", config.ignore_case);
        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{}", line)
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::cli;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            cli::search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            cli::search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            cli::search_case_insensitive(query, contents)
        );
    }
}
