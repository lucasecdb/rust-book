use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let case_sensitive = config.case_sensitive;

    let results = search(&config.query, &contents, |query, line| {
        if case_sensitive {
            line.contains(query)
        } else {
            line.to_lowercase().contains(&query.to_lowercase())
        }
    });

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a, T>(query: &str, contents: &'a str, mut comparator: T) -> Vec<&'a str>
where
    T: FnMut(&str, &'a str) -> bool,
{
    contents
        .lines()
        .filter(|line| comparator(query, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, |query, line| line.contains(query))
        );
    }

    #[test]
    fn constant_condition() {
        let query = "abc";
        let contents = "\
Some
Random
Lines.";

        assert_eq!(
            vec!["Some", "Random", "Lines."],
            search(query, contents, |_, _| true)
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
            search(query, contents, |query, line| line
                .to_lowercase()
                .contains(&query.to_lowercase()))
        );
    }
}
