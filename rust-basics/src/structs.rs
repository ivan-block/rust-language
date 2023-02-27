struct Colour {
    red: u8,
    green: u8,
    blue: u8
}

pub fn run() {
    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Colour is: {} {} {}", c.red, c.green, c.blue);
}