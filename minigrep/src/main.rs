use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else allows to define custom non-panic! error handling;
    // If the Result is an Ok value, this method’s behavior is similar to
    // unwrap: it returns the inner value Ok is wrapping. However, if the
    // value is an Err value, this method calls the code in the closure,
    // which is an anonymous function we define and pass as an argument
    // to unwrap_or_else.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Box<dyn Error> means the function will return a type that implements the
    // Error trait, but we don’t have to specify what particular type the
    // return value will be; dyn keyword is short for “dynamic.”;
    fn new(args: &[String]) -> Result<Config, &'static str> {
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
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(()) // returns "execute";
}
