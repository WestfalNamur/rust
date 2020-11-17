#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method  // 2x methods
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Associated Functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    // Associated functions call
    let sqr = Rectangle::square(2);

    // Method call  // Within println!
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&sqr));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
