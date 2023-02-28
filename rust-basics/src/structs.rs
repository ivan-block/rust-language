// struct Colour {
//     red: u8,
//     green: u8,
//     blue: u8
// }
#[derive(Debug)]
struct Rectangle {
    length: u64,
    width: u64
}

fn area(area: Rectangle) -> u64 {
    area.length * area.width
}

pub fn run() {
    // let c = Colour {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // println!("Colour is: {} {} {}", c.red, c.green, c.blue);
    let scale = 2;
    let area1 = Rectangle {
        length: dbg!(30 * scale),
        width: 30
    };
    dbg!(area1);
    // println!("The area of the rectangle is {:?}", area(area1));

}