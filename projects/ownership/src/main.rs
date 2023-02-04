fn main() {
    // // let mut s = String::from("Hello");
    // // s.push_str(", world");
    // let s2 = String::from("Hello");
    // let s1 = s2.clone();
    // // println!("{}, world", s1);
    // println!("{}", s2);

    // Ownerships and functions
    // let s = String::from("Hello");
    // take_ownerships(s);
    // let x = 5;
    // makes_copy(x);

    // Return values and scopes
    let s1 = gives_ownership();

    println!("{}", s1)
}

// fn take_ownerships(some_strings: String) {
//     println!("{}", some_strings);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

fn gives_ownership() -> String {
    let some_strings = String::from("hey");

    some_strings
}
