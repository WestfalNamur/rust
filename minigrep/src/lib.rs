use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Box<dyn Error> means the function will return a type that implements the
    // Error trait, but we don’t have to specify what particular type the
    // return value will be; dyn keyword is short for “dynamic.”;
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Box<dyn Error> means the function will return a type that implements the
// Error trait, but we don’t have to specify what particular type the return
// value will be;
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// lifetime annotation conncets the return values lifetime to the arguments
// lifetime; indicate that the returned vector should contain string slices
// that reference slices of the argument contents (rather than the argument
// query); we tell Rust that the data returned by the search function will
// live as long as the data passed into the search function in the contents
// argument. This is important! The data referenced by a slice needs to be
// valid for the reference to be valid;
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
