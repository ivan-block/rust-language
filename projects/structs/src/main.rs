#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true,
        name: String::from("Ivan Agwuye"),
        email: String::from("agwuyeivan3@gmail.com"),
        sign_in_count: 1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     name: user1.name,
    //     email: user1.email,
    //     sign_in_count: 2
    // };

    // lesser code
    // let user2 = User {
    //     sign_in_count: 1,
    //     ..user1
    // };

    println!("{:#?}", user1);
}
