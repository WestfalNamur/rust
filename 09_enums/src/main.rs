#![allow(unused)]
fn main() {
    // Enumeration ************************************************************
    // Set of values or a way to give a name to a set of values;
    // Helps to define a set of values => What the value can be;
    // enumerate all possible variants;

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Hello, enums!");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
