struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn can_hold(&self) -> u32 {
        self.width * self.height
    }
}
