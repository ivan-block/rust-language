pub fn run() {
    let name = "Ivan";
    let age = 24;

    const ID: u32 = 007;

    println!("My name is {} and i am {}. {}", name, age, ID);

    // Multiple vars
    let (my_name, my_age) = ("Ivan", 24);
    println!("{} is {}", my_name, my_age)
}
