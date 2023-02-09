// using a struct
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // let width1 = 30;
    // let height1 = 30;

    // let dimensions = (30, 30);

    // println!("The area of the rectangle is {} in square pixels", area(width1, height1));
    // println!("The area of the rectangle is {} in square pixels", area(dimensions));

    let rect1 = Rectangle {
        width: 30,
        height: 30
    };

    println!("The area of the rectangle is {} in square pixels", area(&rect1));
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// using single variables
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// using tuples
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


