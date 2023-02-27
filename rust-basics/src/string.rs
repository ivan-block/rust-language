pub fn run() {
    let mut hello = String::from("Hello ");

    // push methods for primitive string and String
    hello.push('W');
    hello.push_str("orld");

    // String capacity
    println!("Capacity is: {}", hello.capacity());

    // Is empty
    println!("{}", hello.is_empty());

    // Contains
    println!("{}", hello.contains("World"));

    // Replace
    println!("{}", hello.replace("World", "There"));

    // Loop through by whitesapace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    println!("{}", hello);
}