fn main() {
    // ************************************************************************
    // const: always immutable, must have type annotation, and valid the entire
    // time a program runs, within the scope they were declared in;
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // varibable declared as mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    // mutating the variable
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    // lets us change the type of the varibale;
    // shadowing can be avoided using const;
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {}", spaces);

    // ************************************************************************
    // behind basic types
    // Compound types: Compound types can group multiple values into one type.
    // tuple
    // A tuple is a general way of grouping together a number of values with a
    // variety of types into one compound type. Tuples have a fixed length: once
    // declared, they cannot grow or shrink in size;
    let tup: (i32, f64, u8) = (500, 5.3, 1); // construct tuple
    let (_x, y, _z) = tup; // destructure
    println!("The value of y is: {}", y);
    println!("The last value of tup is: {}", tup.2); // access values by indes

    // array
    // every element of an array must have the same type;
    // arrays in Rust have a fixed length;
    // useful when you want your data allocated on the stack rather than the heap;
    // or when you want to ensure you always have a fixed number of elements;
    // An array is a single chunk of memory allocated on the stack;
    // let a = [1, 2, 3, 4, 5];
    // let arr0: [i32; 4] = [0, 1, 2, 3]; // array of 4 i32 values
    // let arr1 = [3; 5]; // array of 3x 5;
    let arr: [i32; 4] = [1, 2, 4, 8];
    let e = arr[3];
    println!("The value of element is: {}", e);
}
