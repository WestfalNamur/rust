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
            return Err("Not engough arguments!");
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

    println!("With text:\n{}", contents);

    Ok(()) // returns "execute";
}
