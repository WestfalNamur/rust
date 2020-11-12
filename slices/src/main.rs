fn main() {
    // ************************************************************************
    // A potential bug
    let mut s1 = String::from("hello world");

    let _word = first_word_v1_err(&s1); // _word is not bound to underlying data;

    s1.clear(); // ! this empties the String, making it equal to ""
                // ! word still has the value 5 here, but there's no more string
                // ! that we could meaningfully use the value 5 with. word is
                // ! now totally invalid!

    // ************************************************************************
    // * Slices Type: Does not have ownership; Let you reference a continguouse
    // * sequence of elements in a collection;
    // *** String Slice ***
    // Reference to part of a String;
    let s2 = String::from("hello world");

    let _slice = &s2[6..];

    let _hello = first_word_v2_no_bug(&s2); // bound to undelying data;

    // s2.clear() is not possible: 1st s2 is not mutable 2nd it would mutate
    // the undelying data to which there are borrows;

    println!("the first word is: {}", _hello);

    // *** Other slices ***
    let a = [1, 2, 3, 4, 5];
    let _a_slice = &a[1..2];
}

fn first_word_v1_err(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2_no_bug(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
