fn main() {
    // ************************************************************************
    // if
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let conditon = true;
    let n = if conditon { 5 } else { 6 };
    println!("The value of n is: {}", n);

    // ************************************************************************
    // loop
    // executes over and over again until explicitly tell it to stop;
    // to stop we use break;
    // if we want to return a value (or expression that evaluates to one), we
    // put it behind break;
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);

    // conditional loop
    // loop continues until condition evaluates to false;
    let mut x = 3;
    while x != 0 {
        println!("x: {}", x);
        x -= 1;
    }
    println!("done");

    // loop through collection
    // loop through a collection in a while loop is errorprone, we coule write
    // a loop that index out of bound of the array;
    // it is also slow as the compiler add runtime code to perform condition
    // checks on every element on every iteration through the loop;
    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("value is: {}", a[i]);
        i += 1;
    }

    // iterate
    // we iterate through the collection;
    // it reduces chances of bugs (i.e. out of bound);
    for e in a.iter() {
        println!("value is: {}", e);
    }

    // for range
    // loop backwards (.rev()), over a range of values;
    for ni in (1..4).rev() {
        println!("value is: {}", ni);
    }
}
