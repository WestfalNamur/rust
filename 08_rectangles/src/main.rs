#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Usage of struct as width and height are related data
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print!("rect1 is {:?}\n", rect1);

    print!("rect1 is {:#?}\n", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
