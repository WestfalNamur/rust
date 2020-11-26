fn main() {
    hello_fn();
    paremeter_fn(6);
    body_fn();
    let x = return_fn(4);
    println!("The value of x is: {}", x);
}

fn hello_fn() {
    // simple function with no parameters;
    println!("Hello, functions!");
}

fn paremeter_fn(x: i32) {
    // function with a paramter;
    // in function signatures, you must declare the type of each parameter;
    println!("Values of x is: {}", x);
}

fn body_fn() {
    // Function bodies are made up of a series of statements optionally ending
    // in an expression;

    // Statements are instructions that perform some action and do not return a
    // value; Therefore, you can’t assign a let statement to another variable;
    let x = 5; // statement

    // Expressions evalute to something;
    // Expressions do not include ending semicolons. If you add a semicolon to
    // the end of an expression, you turn it into a statement, which will then
    // not return a value.
    let y = {
        let z = 5;
        z + 1
    };

    println!("Values of x is: {}", x);
    println!("Values of y is: {}", y);
}

fn return_fn(x: i32) -> i32 {
    // the return value of the function is synonymous with the value of the
    // final expression in the block of the body of a function;
    x + 1
}
