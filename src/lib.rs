use std::{ fs, env };
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        args: &[String]
    ) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // If IGNORE_CASE is in env variables
        let ignore_case = env
            ::var("IGNORE_CASE")
            .is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    // The lifetime 'a ensure that the data
    // returned by the search function will
    // live as long as the content data
    // passed in.

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    // The lifetime 'a ensure that the data
    // returned by the search function will
    // live as long as the content data
    // passed in.

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(
    config: Config
) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(
        config.file_path
    )?;

    let results = if config.ignore_case {
        search_case_insensitive(
            &config.query,
            &contents
        )
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents =
            "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents =
            "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents =
            "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(
                query,
                contents
            )
        )
    }
}