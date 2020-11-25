#![allow(unused)]
fn main() {
    // ************************************************************************
    // * Match *
    // Compare a value against patterns and execute code based on mathing ones
    // The compiler confirms that all possible cases are handled;

    #[derive(Debug)]
    enum UsState {
        Alabame,
        Alaska,
        // ...
    }

    enum Coin {
        Penny,
        Nickle,
        Dime,
        Quarter(UsState),
    }

    // * Match ****************************************************************
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => 25,
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // Other
    }

    // * if let ***************************************************************
    // if let means less typing, less indentation, and less boilerplate code.
    // However, you lose the exhaustive checking that match enforces.
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
