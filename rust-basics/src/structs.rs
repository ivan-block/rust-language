struct Colour {
    red: u8,
    green: u8,
    blue: u8
}
#[deriv(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u8
}

fn build_user(email: String, username: String) -> User {
    username,
    email,
    sign_in_count: 1
}

pub fn run() {
    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Colour is: {} {} {}", c.red, c.green, c.blue);

    let user1 = build_user (
        String::from("ivanagwuye@gmail.com"),
        String::from("ivanagwuye")
    );

    println!("My username is {:?}", user1);
}