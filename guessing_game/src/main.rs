use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // create random number i32 between 1 and 100
    // rand::thread_rng guves us a random number local to the current thread of
    // execution and seeded by the operating system;
    // get_range defines the range of the random number;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter your guess.");

        // let statement to create a variable to store the usr input bound to guess;
        // mut statement to indicate that it should be a mutable variable;
        // guess is bould to the result of calling String::new which returns a new
        // instance of String, a type of UTF-8 encoded text;
        // :: syntax indicates that new is an associated function of the String
        // type: String::new();
        // Createing a mutable variable that is currently bound to a new empty
        // instance of a String;
        let mut guess = String::new();

        // call the stdin function from io module to handle user input;
        // .read_line(buf: &mut guess) methode on the standard input handle. We pass
        // it a reference to the guess. It places what ever the users intpus into a
        // string. The string arg must be mutable so the methode change the strings
        // conent;
        // & indiacetes that the argument is a reference;
        // mut & indicates that it is a reference that allows mutation of the
        // original object;
        // .expect() handels the Result type, an enumeration of either Ok or Err.
        // Ok indicats that the operation was successfull while Err indicates
        // something went wrong and containes informeation about what;
        // If this instance of io::Result is an Err value, expect will cause the
        // program to crash and display the message that you passed as an argument
        // to expect;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // conver the guess to a u32 for later comparions;
        // We shadow the old varibale with a new one which is pssible as we
        // instantiated a variable with let;
        // .trim() eliminates whitespaces on a string;
        // .parse() parses a string into a number;
        // We match the Result to either bind is to guess if is convertable to
        // u32 or to continue (loop further) to prompt for user input again if
        // an Err is returned;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // math cases of Ordering of guess to against random number;
        // Ordering Enum retunrs Less, Greater and Equal when applied to a value
        // and passed an argument;
        // If guess the correct number, break out of the loop;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To low!"),
            Ordering::Greater => println!("To high!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
