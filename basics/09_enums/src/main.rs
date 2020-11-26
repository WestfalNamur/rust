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

    // ************************************************************************
    // * Option<T> *
    // Enum that can encode the concept of a value not being present or 
    // absent.
    // <T>: generic type parameter
    // Option<T> enum is still just a regular enum, and Some(T) and None are
    // still variants of type Option<T>;
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;


}
