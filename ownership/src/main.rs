fn main() {
    // ************************************************************************
    // Ownership
    // - Memory in Rust is managed through a system of ownership with a set of
    //   rules that the compiler checks at compile time;
    // - Stack and Heap
    //   - Stack: last in, first out; fast; data must have known and fixed size;
    //     Stack allocations are only one CPU instruction, moving the stack
    //     pointer and immediatley return the address of the stack pointer;
    //     Data pops off the stack when going out of scope;
    //     Data is closer together;
    //     When your code calls a function, the values passed into the function
    //     (including, potentially, pointers to data on the heap) and the
    //     function’s local variables get pushed onto the stack. When the
    //     function is over, those values get popped off the stack;
    //     Data with an unknown size at compile time or a size that might change
    //     must be stored on the heap instead;
    //   - Heap: less structured; allocating memory involves several steps
    //     including bookkeeping, ask the OS for space in memory, and others.
    // - Rules:
    //   1 Each value in Rust has a variable that’s called its owner.
    //   2 There can only be one owner at a time.
    //   3 When the owner goes out of scope, the value will be dropped.
    // ************************************************************************
    //
    // Variable scope
    //
    {
        // Start of scope;
        // String literal: size known at compile time and fixed, thus stored on the
        // stack; Immutable;
        let s1 = "Hello, world!";
        // End of scope: s is droped;
        println!("{}", s1);
    }

    //
    // String Type and String literal, heap and stack;
    //
    {
        // String Type: unknown size at compile and hence stored on the heap;
        // Is mutable;
        let mut s2 = String::from("Hello!");
        s2.push_str(", world!");
        println!("{}", s2);
    } // s2, the owner of String gets out of scope so the alloced memory of
      // the string is freed;

    //
    // Move & clone
    //
    // x and y are simple integers with known size at compile time and fixed;
    // They get pused on the stack; y is a copy or "clone" of x; This is
    // possible with types that have the clone trait;
    let x = 5;
    let y = x;
    println!("{}", y);
    // s3's content is stored on the heap, its pointer on the stack;
    // Two things happen here: 1 we copy the data of s3 to s4, the stack data
    // thus the pointer, not the conent; 2 as each value can only have one owner
    // we drop s3; This is called a "move"; We move s3 into s4;
    let s3 = String::from("I am moved and cloned!");
    let s4 = s3;
    // we can copy the content, as this is expensive it is not the default so
    // we have to express our intent; We "clone" s3;
    let s5 = s4.clone();
    println!("{}", s5);

    // ************************************************************************
    //
    // Ownership and functions
    // Passing a variable to a function will move or copy, just as assignment does;
    //
    // We pass a String (content on heap, pointer on stack) to the function;
    // The function takes ownership of the string; s6 is this no longer valid;
    let s6 = String::from("Hello!");
    takes_ownership(s6);
    // We pass an int32 (known size and immutable, stored on the stack) to the
    // function, a copy is made. x is still valid;
    let x = 5;
    makes_copy(x);
    //
    // Return values and scope
    // Last expression of function is returned and bound to _s7;
    let _s7 = gives_ownership();
    //
    // s8 is passed to fn which takes ownership; s8 is droped; The last
    // expression is returned and bound to _s9;
    let s8 = String::from("Hello!");
    let _s9 = takes_and_gives_ownership(s8);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(int: i32) {
    println!("{}", int);
}

fn gives_ownership() -> String {
    let s10 = String::from("hello");
    s10
}
fn takes_and_gives_ownership(str: String) -> String {
    str
}
