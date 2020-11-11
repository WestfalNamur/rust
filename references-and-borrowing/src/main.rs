fn main() {
    let s1 = String::from("Hello!");

    // Reference rules
    // At any given time, you can have either one mutable reference or any
    // number of immutable references. References must always be valid.

    // passing a value by "reference" indicatied by &;
    // &s1 is a pointer to s1 pointer on the stack which owns and points to the
    // content of s1; This references does not own the variable and hence the
    // value will not be droped when &s1 moves out of scope;
    // the values is "borrowed";
    let len = calc_len(&s1);

    // mutable reference;
    // you can have only one mutable reference to a particular piece of data
    // in a particular scope. This code will fail;
    let mut s2 = String::from("Hello!");
    {
        // This works as it is a different scope;
        let _r1 = &mut s2;
    }
    change(&mut s2);

    // works as well as r1 and r2 are dopped before r3 gets into scope;
    let mut s3 = String::from("hello");
    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s3; // no problem
    println!("{}", r3);

    println!("The length of '{}' is {}.", s1, len);

    // Dangline references
    let _reference_to_nothing = dangle();
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    // if we would return &s it would be dangling as s will be droped when the
    // functions end. This &s would be a reference to nothing;
    s
}
