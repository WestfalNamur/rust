use std::collections::HashMap;

fn main() {
    // *** vector ***
    {
        // let v: Vec<i32> = Vec::new(); or using a macro
        let mut v = vec![1, 2, 3];

        v.push(4);
        v.push(5);
        v.push(6);

        // Get a value: with reference => can panic
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        // Get a value: by getter => Need to handel None case;
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }

        // v and all it's elements will be droped
    }

    // iterate over elements in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using enums to store different types in the same vector
    // vectors can only store values that are the same type
    // the variants of an enum are defined under the same enum type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // *** hash map ***
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Grey")).or_insert(50);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
