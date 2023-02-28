struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
    }
}

pub fn run() {
    let rect1 = Rectangle {
    length: 5,
    width: 30
    };

    let rect2 = Rectangle {
    length: 10,
    width: 40
    };

    let _rect3 = Rectangle {
    length: 60,
    width: 45
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("The area of the rectangle is {}", rect1.area());
}