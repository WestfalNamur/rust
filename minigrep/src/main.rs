use std::env;
use std::process;

use minigrep::Config;

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

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
